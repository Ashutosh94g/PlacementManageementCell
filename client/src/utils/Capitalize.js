function Capitalize(sentence) {
    let words = sentence.split(" ");
    let new_sentence = [];
    for(let word of words){
        let new_word = word[0].toUpperCase() + word.slice(1);
        new_sentence.push(new_word);
    }

    return new_sentence.join(" ");
}

export default Capitalize;