Stenome
#######

Active SRS learning.

Probably the most common Open Source SRS learning application is Anki.
Anki works well.  Stenome is also an application for doing SRS
learning, but it has a focus on learning things that involve
specialized input devices.  It was originally written for learning to
use a Steno keyboard (based on information from the Plover Dojo).  It
has since been aquiring MIDI input support to help learn scales and
chord voicings.

The MIDI support is not well documented yet, as it is still under
heavy development.

Steno
=====

The Plover Dojo is a great way to learn Plover/Steno.  However, the
Dojo seems to be lacking in support for SRS (spaced repetition), and
as more and more words are learned, it is easy to begin to forget
perviously learned words.

Stenome attempts to combine the learning aspect of the Dojo with some
knowledge of SRS so that what has been learned will not be forgotten.

It incorporates the word lists from the Plover Dojo, and it is
probably best to just use the Dojo to learn the keyboard itself.

Setup
=====

Stenome is a fairly straightforward Rust application, and should be
buildable with ``cargo build``, or ``cargo install`` to install the
binary.  It has been tested on MacOS and Linux, and its requirements
are fairly simple, mainly on ``termion`` for the raw terminal
functionality.

Unlike the Plover Dojo, Stenome depends on Plover for steno keyboard
support.  This allows us to support any keyboard supported by Plover,
and also avoids us having to duplicate any code.

You will need to disable the translation dictionaries in Plover to be
able to run Stenome.  Within Plover, select "Configure...", and then
the "Dictionary" tag.  You should have a user.json dictionary listed,
and make sure this is blank.  If it is not, create a new dictionary,
as removing all dictionaries is not possible in Plover.  Once you have
this empty user dictionary, remove all other dictionaries.  It is easy
to restore the dictionaries.  Simply remove all dictionaries, and
Plover will restore the default set of dictionaries.

The other option that needs to be changed is under "Output", setting
the "Space Placement" to "After Output".  This is needed for Stenome
to be able to know right away when a stroke has finished.

Running
=======

The first time you run Stenome, it will create a learning.json that
describes the learning state, with no words see.  It will then prompt
with the English and wait for you to write this word.  You can press
Escape at any time to save the current state to learning.json and exit
the program.

Otherwise, try to write the word shown.  If you don't know how, press
something incorrect, and use '*' on the writer to erase it.  As soon
as you make a mistake, Stenome will show the correct strokes and give
you a chance to write the word correctly.

The more you get a word correct, the longer the interval will be
before you are asked again.  The interval will be shortened if you
make mistakes.  New words will be introduced when there are no words
due to be asked.  Over time you should build up a list of words at
various intervals.  Ideally, regular practice should move a large
number of words to long intervals (such as > 1 month), and these words
can be considered to be known.  Continuing to periodically run stenom
will refresh your knowledge of words.
