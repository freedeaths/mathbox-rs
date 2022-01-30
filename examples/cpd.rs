use mathbox::app::signal::change_points::e_divisive;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut data_1 = vec![];
    let section_1 = [100.0; 100];
    let section_2 = [3000.0; 100];
    let section_3 = [200.0; 100];
    let section_4 = [3000.0; 100];
    let section_5 = [4000.0; 100];
    let section_6 = [500.0; 100];
    data_1.extend_from_slice(&section_1);
    data_1.extend_from_slice(&section_2);
    data_1.extend_from_slice(&section_3);
    data_1.extend_from_slice(&section_4);
    data_1.extend_from_slice(&section_5);
    data_1.extend_from_slice(&section_6);
    data_1.iter_mut().for_each(|x| *x += rng.gen_range(0.0..3.0));

    let cp = e_divisive(&data_1, 7, 0.05, 100);
    println!("{:?}", cp);
}
