use sdts::filter::FilterCreator;
use sdts::filter::Filter;

#[test]
pub fn straight_through() {
    let mut filt1 = FilterCreator::<1,1>::create_filter(|x,y|{
        y[0] = x[0];
    });

    let xin = [1.0,2.0,3.0,4.0,5.0];
    let mut yout = [0.0,0.0,0.0,0.0,0.0];

    for i in 0..5{
        yout[i] = filt1.filt(xin[i]);
    }

    assert_eq!(yout, xin);
}
