pub(crate) fn b(value: &(f64, [f64; 4], Vec<crate::dovecote::DoveCote>)) -> (f64, f64, Vec<f64>) {
    let x = value.0;
    let avg = value.1[0];
    let mut vals = Vec::new();
    value.2.iter().for_each(|dovecote| {
        vals.push(dovecote.first_collision as f64);
    });
    return (x, avg, vals);
}

pub(crate) fn u(value: &(f64, [f64; 4], Vec<crate::dovecote::DoveCote>)) -> (f64, f64, Vec<f64>) {
    let x = value.0;
    let avg = value.1[1];
    let mut vals = Vec::new();
    value.2.iter().for_each(|dovecote| {
        vals.push(dovecote.empty_boxes as f64);
    });
    return (x, avg, vals);
}

pub(crate) fn c(value: &(f64, [f64; 4], Vec<crate::dovecote::DoveCote>)) -> (f64, f64, Vec<f64>) {
    let x = value.0;
    let avg = value.1[2];
    let mut vals = Vec::new();
    value.2.iter().for_each(|dovecote| {
        vals.push(dovecote.all_one as f64);
    });
    return (x, avg, vals);
}

pub(crate) fn d(value: &(f64, [f64; 4], Vec<crate::dovecote::DoveCote>)) -> (f64, f64, Vec<f64>) {
    let x = value.0;
    let avg = value.1[3];
    let mut vals = Vec::new();
    value.2.iter().for_each(|dovecote| {
        vals.push(dovecote.all_two as f64);
    });
    return (x, avg, vals);
}

pub(crate) fn dc(value: &(f64, [f64; 4], Vec<crate::dovecote::DoveCote>)) -> (f64, f64, Vec<f64>) {
    let x = value.0;
    let avg = value.1[3] - value.1[2];
    let mut vals = Vec::new();
    value.2.iter().for_each(|dovecote| {
        vals.push((dovecote.all_two - dovecote.all_one) as f64);
    });
    return (x, avg, vals);
}