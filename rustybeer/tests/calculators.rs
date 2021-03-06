use rustybeer_util::assert_approx;

#[test]
fn abv() {
    use rustybeer::calculators::abv::calculate_abv;

    assert_approx!(1050., calculate_abv(10., 2.));

    assert_approx!(39.5548, calculate_abv(0.3026, 0.00123));
}

#[test]
fn boil_off() {
    use rustybeer::calculators::diluting::*;

    assert_approx!(2., calculate_new_volume(2., 2., 2.));

    assert_approx!(15., calculate_new_volume(7., 5., 3.));

    assert_approx!(11., calculate_new_gravity(7., 5., 3.));

    assert_approx!(69.5714, calculate_new_gravity(4., 3.2, 0.14));
}

#[test]
fn diluting() {
    use rustybeer::calculators::diluting::*;

    assert_approx!(14.1625, calculate_new_gravity(9.1, 5.2, 3.2));

    assert_approx!(4.5305, calculate_new_gravity(9.1, 3.16, 7.25));

    assert_approx!(2.0, calculate_new_volume(2., 2., 2.));

    assert_approx!(15., calculate_new_volume(7., 5., 3.));
}

#[test]
fn priming() {
    use rustybeer::calculators::priming::{calculate_co2, calculate_sugars, Sugar};

    assert_approx!(2.3466, calculate_co2(15.));

    assert_approx!(2.4556, calculate_co2(12.45));

    let stream = calculate_sugars(77., 5., 2.);

    let expected = vec![
        Sugar::new(String::from("Table Sugar (sucrose)"), 24.850561000000013),
        Sugar::new(String::from("Corn Sugar (dextrose)"), 27.308308791208802),
        Sugar::new(String::from("DME - All Varieties"), 36.54494264705884),
        Sugar::new(String::from("DME - Laaglander"), 49.701122000000026),
        Sugar::new(String::from("Turbinado"), 24.850561000000013),
        Sugar::new(String::from("Demarara"), 24.850561000000013),
        Sugar::new(String::from("Corn Syrup"), 36.015305797101476),
        Sugar::new(String::from("Brown Sugar"), 27.92197865168541),
        Sugar::new(String::from("Molasses"), 35.00079014084509),
        Sugar::new(String::from("Maple Syrup"), 32.27345584415586),
        Sugar::new(String::from("Sorghum Syrup"), 36.015305797101476),
        Sugar::new(String::from("Honey"), 33.5818391891892),
        Sugar::new(String::from("Belgian Candy Syrup"), 39.44533492063494),
        Sugar::new(String::from("Belgian Candy Sugar"), 33.13408133333335),
        Sugar::new(String::from("Invert Sugar Syrup"), 27.308308791208802),
        Sugar::new(String::from("Black Treacle"), 28.563863218390818),
        Sugar::new(String::from("Rice Solids"), 31.45640632911394),
    ];

    assert_eq!(expected, stream);
}

#[test]
fn sg_correction() {
    use rustybeer::calculators::sg_correction::correct_sg;

    assert_approx!(5.001, correct_sg(5.0, 2.9, 1.37));

    assert_approx!(7.3023, correct_sg(7.3, 8.1, 5.12));

    assert_approx!(7.4175, correct_sg(7.413, 28.1, 55.1212));
}
