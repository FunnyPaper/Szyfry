use szyfry::utils::*;

#[test]
fn parse_code_1() {
    let code: Key = Key::read("translation_keys/test.ckey", KeyRule::Column).unwrap();
    assert_eq!(code.to_string(), r#";dPqy($epXwQ0/v+=HW#mx.B6s`Nt)1Ar-z4|u8G^jEOJ,3Kl7@b
 a  R      !{kn    9& YZ hS >gV_  Ci\ 5"*} %UcMT    
 '  o        FD    <  Lf    I           :  ?] 2[    "#);
}

#[test]
fn parse_code_2() {
    let code: Key = Key::read("translation_keys/test.lkey", KeyRule::Row).unwrap();
    assert_eq!(code.to_string(), r#"Y}=
wR*
bD
S#M
su
%
/
Px
oC
F4
`g,
OA
aGt
y>E
85
6B
flc
W
9
7^T
1I
e
2
Uj
.
3
H
@
vNK
_kQ
\
<(|
{-?
iZ)
;
p
X
m]J
&V0
[
$n
:
z
d
+
q
L
r
'
!
h
""#);
}