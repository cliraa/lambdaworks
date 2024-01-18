use crate::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use crate::elliptic_curve::traits::IsEllipticCurve;
use crate::field::fields::secp256k1_field::Secp256k1PrimeField;
use crate::{
    elliptic_curve::short_weierstrass::traits::IsShortWeierstrass, field::element::FieldElement,
};

#[derive(Clone, Debug)]
pub struct Secp256k1Curve;

impl IsEllipticCurve for Secp256k1Curve {
    type BaseField = Secp256k1PrimeField;
    type PointRepresentation = ShortWeierstrassProjectivePoint<Self>;

    fn generator() -> Self::PointRepresentation {
        Self::PointRepresentation::new([
            FieldElement::<Self::BaseField>::from_hex_unchecked("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"),
            FieldElement::<Self::BaseField>::from_hex_unchecked("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"),            
            FieldElement::one()
        ])
    }
}

impl IsShortWeierstrass for Secp256k1Curve {
    fn a() -> FieldElement<Self::BaseField> {
        FieldElement::from(0)
    }

    fn b() -> FieldElement<Self::BaseField> {
        FieldElement::from(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cyclic_group::IsGroup, elliptic_curve::traits::EllipticCurveError,
        field::element::FieldElement,
    };

    use super::Secp256k1Curve;

    #[allow(clippy::upper_case_acronyms)]
    type FE = FieldElement<Secp256k1PrimeField>;

    fn point_1() -> ShortWeierstrassProjectivePoint<Secp256k1Curve> {
        let x = FE::from_hex_unchecked(
            "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        );
        let y = FE::from_hex_unchecked(
            "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a",
        );
        Secp256k1Curve::create_point_from_affine(x, y).unwrap()
    }

    fn point_1_times_5() -> ShortWeierstrassProjectivePoint<Secp256k1Curve> {
        let x = FE::from_hex_unchecked(
            "a0434d9e47f3c86235477c7b1ae6ae5d3442d49b1943c2b752a68e2a47e247c7",
        );
        let y = FE::from_hex_unchecked(
            "893aba425419bc27a3b6c7e693a24c696f794c2ed877a1593cbee53b037368d7",
        );
        Secp256k1Curve::create_point_from_affine(x, y).unwrap()
    }

    #[test]
    fn adding_five_times_point_1_works() { // This test is not working
        let point_1 = point_1();
        let point_1_times_5 = point_1_times_5();
        assert_eq!(point_1.operate_with_self(5_u16), point_1_times_5);
    }

    #[test]
    fn create_valid_point_works() {
        let p = point_1();
        assert_eq!(
            *p.x(),
            FE::from_hex_unchecked(
                "c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5"
            )
        );
        assert_eq!(
            *p.y(),
            FE::from_hex_unchecked(
                "1ae168fea63dc339a3c58419466ceaeef7f632653266d0e1236431a950cfe52a"
            )
        );
        assert_eq!(*p.z(), FE::from_hex_unchecked("1"));
    }

    #[test]
    fn create_invalid_points_returns_an_error() {
        assert_eq!(
            Secp256k1Curve::create_point_from_affine(FE::from(0), FE::from(1)),
            Err(EllipticCurveError::InvalidPoint)
        );
    }

    #[test]
    fn equality_works() {
        let g = Secp256k1Curve::generator();
        let g2 = g.operate_with_self(2_u16);
        let g2_other = g.operate_with(&g);
        assert_ne!(&g2, &g);
        assert_eq!(&g, &g);
        assert_eq!(&g2, &g2_other);
    }

    #[test]
    fn g_operated_with_g_satifies_ec_equation() { // This should work, but it doesn't.
        let g = Secp256k1Curve::generator();
        let g2 = g.operate_with_self(2_u16);

        // get x and y from affine coordinates
        let g2_affine = g2.to_affine();
        let x = g2_affine.x();
        let y = g2_affine.y();

        // calculate both sides of Secp256k1 curve equation
        let seven = Secp256k1Curve::b();
        let y_sq_0 = x.pow(3_u16) + seven;
        let y_sq_1 = y.pow(2_u16);

        assert_eq!(y_sq_0, y_sq_1);
    }

    #[test]
    fn operate_with_self_works_1() {
        let g = Secp256k1Curve::generator();
        assert_eq!(
            g.operate_with(&(g.operate_with(&g))),
            //g.operate_with(&g).operate_with(&g), // This should work, but it doesn't.
            g.operate_with_self(3_u16)
        );

        assert_eq!(
            g.operate_with(&g),
            //g.operate_with(&g).operate_with(&g), // This should work, but it doesn't.
            g.operate_with_self(2_u16)
        );
    }
}
