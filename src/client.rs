
/* So that accidental poetry may continue to be detected
 * 
 * Data structures:
 * 
 * Self balancing trees are used for individual lookup effeciency
 * WordList BtreeMap<Word, Vec<Pronunciations>> Matches up to a singular word in the dictionary and provides a vector of the pronunciations
 * 
 * Pronunciations Vec<Phonemes> Series of phonemes that make up the pronunciation of a word
 * 
 * Possible built in functions:
 * Detect if word is found in dictionary
 * Return phoneme array for pronunciations of words
 * Counting syllables in the pronunciation of a word
 * Determining if two pronunciations rhyme/Return all rhyming words 
 * 
 * 
 */

use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use phonemes::Phoneme;

pub struct CmuClient {
    word_list: BTreeMap<String, Vec<Vec<Phoneme>>>
}

impl CmuClient {
    //Test for btrees
    pub fn insert_word(&mut self, w: String, p:Vec<Vec<Phoneme>>) {
        self.word_list.insert(w, p);
    }
    //Where we will take the resource file and convert it into our b-tree
    pub fn new(&mut self, location: File){
        self.word_list.clear();
    }
}
