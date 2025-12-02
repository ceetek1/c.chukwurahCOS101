use std::io::Write;

fn main (){
    let  larger = vec![
    "33 Export".to_string(),
    "Desperados".to_string(),
    "Goldberg".to_string(),
    "Gulder".to_string(),
    "Hineken".to_string(),
    "Star".to_string()
    ];

    let stout = vec![
    "Legend".to_string(),
    "Turbo King".to_string(),
    "Williams".to_string()
    ];

    let non_alcoholic = vec![
    "Maltina".to_string(),
    "Amstel Malt".to_string(),
    "Malta Gold".to_string(),
    "Fanta".to_string()
    ];


    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all("Categories of drinks\n".as_bytes()).unwrap();


    write_category(&mut file, "Larger", &larger);
    write_category(&mut file, "Stout",&stout);
    write_category(&mut file, "non_alcoholic",&non_alcoholic);
    println!("\n Data written to file");

    fn write_category (file: &mut std::fs::File,name: &str,items: &Vec<String>){
        file
            .write_all(format!("{}\n{}\n\n", name, items.join("\n")).as_bytes())
            .unwrap();
   }
}