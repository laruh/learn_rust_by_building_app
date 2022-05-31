// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_title(title: Tile) {
    use Tile::*;
    match title {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(other) => println!("{:?}", other),
        Water(pressure) if pressure.0 >= 10 => println!("High water pressure!"),
        Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {:?}", pressure),
        Grass | Dirt | Sand => println!("Ground tile"),
        Treasure(TreasureChest { amount, .. }) if amount >= 100 => println!("Lots of gold!"),
        _ => println!("Average title"),
    }
}

fn main() {
    let title_dungeon = Tile::Brick(BrickStyle::Dungeon);
    let title_water = Tile::Water(Pressure(12));
    let title_treasure = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 114,
    });
    let title_sand = Tile::Sand;
    print_title(title_dungeon);
    print_title(title_water);
    print_title(title_treasure);
    print_title(title_sand);
}
