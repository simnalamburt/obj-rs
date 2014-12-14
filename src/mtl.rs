use lex::lex;

pub fn mtl<T: Buffer>(input: &mut T) {
    lex(input, |stmt, args| {
        match stmt {
            // Material name statement
            "newmtl" => {}

            // Material color and illumination statements
            "Ka" => {}
            "Kd" => {}
            "Ks" => {}
            "Ke" => {}
            "Km" => {}
            "Ns" => {}
            "Ni" => {}
            "Tr" => {}
            "Tf" => {}
            "illum" => {}
            "d" => {}

            // Texture map statements
            "map_Ka" => {}
            "map_Kd" => {}
            "map_Ks" => {}
            "map_d" => {}
            "map_aat" => {}
            "map_refl" => {}
            "map_bump" | "map_Bump" | "bump" => {}
            "disp" => {}

            // Reflection map statement
            "refl" => {}

            // Unexpected statement
            _ => panic!("Unexpected statement: {} {}", stmt, args.connect(" "))
        }
    });
}
