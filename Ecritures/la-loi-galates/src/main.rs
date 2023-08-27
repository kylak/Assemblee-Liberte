use std::fs;

fn main() -> std::io::Result<()> {

    for file in fs::read_dir("source").unwrap() {

        let fichier = format!("{}", file.unwrap().path().display());
        let contents = std::fs::read_to_string(fichier).unwrap();
       
        let infos : Vec<&str> = contents.splitn(8, ':').collect();
        let infos_nom : Vec<&str> = infos[6].splitn(2, ' ').collect();
        let nom_trad = infos_nom[0];
        let date = infos[5];
        let nom_fichier = format!("-{}-{}", date, nom_trad);
        
        let mut occurence = 0;
        let mut new_file_content = String::from("");

        let mut livre_galates_traduit = false;

        for line in contents.lines() {
            
            if line.contains(":GALATES:") {
                
                livre_galates_traduit = true;

                let low = line.to_lowercase();
                
                if low.contains("la loi")|| low.contains("la loy") || low.contains("la torah") || low.contains("la tora") {
                    
                    let mut v: Vec<&str> = low.matches("la loi").collect();
                    occurence += v.iter().count();

                    v = low.matches("la loy").collect();
                    occurence += v.iter().count();
                    
                    v = low.matches("la torah").collect();
                    occurence += v.iter().count();
                    
                    v = low.matches("la tora").collect();
                    occurence += v.iter().count();
                    
                    new_file_content = new_file_content + line + "\n";
                }
            }
        }
        
        if livre_galates_traduit { 
            fs::write(format!("./result/{}{}", occurence.to_string(), &nom_fichier), new_file_content)?;
        }
    }

    Ok(())
}
