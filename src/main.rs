#[derive(Debug)]
enum CroatianRegions {
    Dalmatia,
    Slavonija,
    Zagreb,
    Lika,
    Istra,
}

struct Wine {
    name: String,
    region: CroatianRegions,
}


impl Wine {

    fn is_supported_regions(&self, r: CroatianRegions) {
        if self.name == "Posip" {
            match r {
                CroatianRegions::Dalmatia => println!("Posip is from Dalmatia"),

                _ => println!("Posip is not suported for {:?}", r),
            }
        }
    }
}
fn main() {
    let new_region2 = CroatianRegions::Lika;

    let wine1 = Wine {
        name: String::from("Posip"),
        region: CroatianRegions::Dalmatia,
    };

    wine1.is_supported_regions(new_region2);
    let region5 = CroatianRegions::Istra;
    wine1.is_supported_regions(region5);

}
