package timelearn

// Counts contains statistics about the current state of problems.
type Counts struct {
	// The number of "Active" problems.  Something is considered
	// active if it is due for learning.
	Active int

	// The number of problems that are being learned, but aren't
	// ready to be asked again.
	Later int

	// The number of problems the user has never been shown
	Unlearned int

	// Counts of all of the problems, groups into histogram
	// buckets based on the learning interval of the problem. The
	// bucket names are a short description of the interval
	Buckets []Bucket
}

// A Bucket is a single histogram bucket describing the number of
// problems of a given category.
type Bucket struct {
	Name  string // Short description of this bucket.
	Count int    // The number of problems in this bucket.
}

// A BucketBin represents a single bucket that will be returned to the
// caller.  Limit is the number of this unit before moving to the next
// bucket.
type bucketBin struct {
	name  string
	limit float64
}

var countBuckets = []bucketBin{
	bucketBin{"sec", 60.0},
	bucketBin{"min", 60.0},
	bucketBin{"hr", 24.0},
	bucketBin{"day", 30.0},
	bucketBin{"mon", 1.0e30},
}

// GetCounts retrieves statistics about the problems available.
func (t *T) GetCounts() (*Counts, error) {
	var unlearned int
	err := t.conn.QueryRow(`
		SELECT COUNT(*)
		FROM probs
		WHERE id NOT IN (SELECT probid FROM learning)`).Scan(&unlearned)
	if err != nil {
		return nil, err
	}

	now := t.now()

	var active int
	err = t.conn.QueryRow(`
		SELECT COUNT (*)
		FROM probs JOIN learning
		WHERE probs.id = learning.probid
		    AND next <= ?`,
		timeToDb(now)).Scan(&active)
	if err != nil {
		return nil, err
	}

	var later int
	err = t.conn.QueryRow(`
		SELECT COUNT (*)
		FROM probs JOIN learning
		WHERE probs.id = learning.probid
		    AND next > ?`,
		timeToDb(now)).Scan(&later)
	if err != nil {
		return nil, err
	}

	// Place each problem into the various buckets.
	interval := 1.0
	prior := 0.0
	var bucks = make([]Bucket, 0, len(countBuckets))
	for _, cbuck := range countBuckets {
		interval *= cbuck.limit
		var count int
		err = t.conn.QueryRow(`
			SELECT COUNT(*)
			FROM probs JOIN learning
			WHERE probs.id = learning.probid
			   AND interval <= ? AND interval > ?`,
			interval, prior).Scan(&count)
		if err != nil {
			return nil, err
		}
		prior = interval
		bucks = append(bucks, Bucket{cbuck.name, count})
	}

	return &Counts{
		Active:    active,
		Later:     later,
		Unlearned: unlearned,
		Buckets:   bucks,
	}, nil
}
