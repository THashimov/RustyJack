use rand::{prelude::SliceRandom, thread_rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Aces,
    Diamonds,
    Hearts
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
    pub img_src: String
}

#[derive(Debug)]
pub struct Deck {
    pub deck: Vec<Card>    
}

#[derive(Debug)]
pub struct Shoe {
    pub shoe: Vec<Deck>
}

impl Card {
    pub fn create_card(value: u8, suit: Suit, img_src: String) -> Card {
        Card {
            value,
            suit,
            img_src
        }
    }   
}


impl Deck {
    pub fn create_deck() -> Deck {
        let mut deck: Vec<Card> = Vec::new();

        let mut value = 2;
        let mut suit = Suit::Clubs;
        let mut path = String::new();

        for _i in 0..4 {
            match _i {
                1 => { suit = Suit::Aces },
                2 => { suit = Suit::Diamonds },
                3 => { suit = Suit::Hearts },
                _ => {}
            };
            
            for _j in 0..13 {
                let img_src = get_img_src_for_card(Some(value), Some(suit));

                if let Some(str) = img_src {
                    path = str
                }

                let card = Card::create_card(value, suit, path.clone());

                value += 1;
                if value > 14 {
                    value = 2
                }
                deck.push(card)
            };
        }

        shuffle_cards(&mut deck);
        
        Deck {
            deck
        }
    }
}

impl Shoe {
    pub fn create_shoe() -> Shoe {
        let mut shoe: Vec<Deck> = Vec::new();

        for i in 0..6 {
            let deck = Deck::create_deck();
            shoe.push(deck)
        };

        Shoe {
            shoe
        }


    }
}





pub fn get_img_src_for_card(value: Option<u8>, suit: Option<Suit>) -> Option<String> {
    let mut path = String::from("./src/assets/");

    match value {
        Some(2)=> {path.push_str("2")},
        Some(3)=> {path.push_str("3")},
        Some(4)=> {path.push_str("4")},
        Some(5)=> {path.push_str("5")},
        Some(6)=> {path.push_str("6")},
        Some(7)=> {path.push_str("7")},
        Some(8)=> {path.push_str("8")},
        Some(9)=> {path.push_str("9")},
        Some(10) => {path.push_str("10")},
        Some(11) => {path.push_str("J")},
        Some(12) => {path.push_str("Q")},
        Some(13) => {path.push_str("K")},
        Some(14) => {path.push_str("A")},
        _ => {return None}
    }

    match suit {
        Some(Suit::Diamonds) => {path.push_str("D.png")},
        Some(Suit::Hearts) => {path.push_str("H.png")},
        Some(Suit::Clubs) => {path.push_str("C.png")},
        Some(Suit::Aces) => {path.push_str("A.png")},
        None => {return None}
    }

    Some(path)
}


pub fn shuffle_cards(deck: &mut Vec<Card>) {
    deck.shuffle(&mut thread_rng());
}