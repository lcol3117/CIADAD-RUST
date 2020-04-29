fn main() {
  let tst_pt1: Vec<f64> = vec![1.0,2.0,3.0];
  let tst_pt2: Vec<f64> = vec![1.0,2.0,4.0];
  let tst_pt3: Vec<f64> = vec![9.0,2.0,3.0];
  let tst_data = vec![tst_pt1,tst_pt2,tst_pt3];
  let res = ciadad(tst_data);
  println!("{:?}", res);
}

fn ciadad(data: Vec<Vec<f64>>) -> Vec<bool> {
  let vals = data
  .iter()
  .map(|x| get_d(x.to_vec(), data.to_vec()))
  .collect::<Vec<f64>>();
  let thresh = median((&vals).to_vec());
  vals
  .iter()
  .map(|x| (x > &thresh))
  .collect::<Vec<bool>>()
}

fn get_d(a: Vec<f64>, data: Vec<Vec<f64>>) -> f64 {
  let alltransform = data
  .iter()
  .map(|x| dist_not_self(a.to_vec(), x.to_vec()))
  .collect::<Vec<f64>>();
  f64min(alltransform)
}

fn dist_not_self(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let raw = cia3d_dist(a, b);
  if raw == 0.0 {
    std::f64::INFINITY
  } else {
    raw
  }
}

fn cia3d_dist(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let deltas = list_deltas(a,b);
  f64max(deltas)
}

fn list_deltas(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
  let length = a.len();
  (0..length)
  .map(|i| (&a[i] - &b[i]).abs())
  .collect::<Vec<f64>>()
}

fn f64max(l: Vec<f64>) -> f64 {
  l
  .iter()
  .cloned()
  .fold(0./0., f64::max)
}

fn f64min(l: Vec<f64>) -> f64 {
  l
  .iter()
  .cloned()
  .fold(0./0., f64::min)
}

fn median(l: Vec<f64>) -> f64 {
  let index_f64 = l.len() / 2;
  let index: usize = index_f64 as usize;
  l[index]
}