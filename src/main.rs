extern crate termion;
extern crate stenome;

// Stenome expects Plover to do the decoding of the steno keyboard.  To make this work, you should
// either have an empty user dictionary, or add an empty dictionary to the list.  Then, remove all
// of the other dictionaries.  This will cause everything to be untranslatable.  Also, change the
// output settings so that the space is sent after the stroke, rather than before.  This allows us
// to decode the raw steno strokes as they are sent.

fn main() {
    stenome::run();
}
