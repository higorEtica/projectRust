pub fn execute(moeda: i32, rodada: i32) -> f32 {
        (1.0 - pk(moeda, rodada)) * 100.0
    }

pub fn pk(rodada: i32, moedas: i32) -> f32 {
        let mut result = 1.0;
        for _x in 1..rodada {
            result = result * 0.50;
        }
        result = 1.0 - result;
        for _y in 1..moedas {
            result = result * result;
        }

        result
    }
