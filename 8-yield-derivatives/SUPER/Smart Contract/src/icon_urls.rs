use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor, Clone)]
pub struct IconUrls {
    pub floww: UncheckedUrl,
    pub super_s: UncheckedUrl,
    pub super_t: UncheckedUrl,
    pub super_y: UncheckedUrl,
    pub ww: UncheckedUrl,
    pub nft: UncheckedUrl,
}

#[derive(NonFungibleData, ScryptoSbor, Clone)]
pub struct Icons {
    pub black: IconUrls,
    pub white: IconUrls,
}

impl Icons {
    pub fn new() -> Self {
        let base_url: String = String::from("https://assets.floww.fi/images/logo/png/");

        let set: Icons = Icons {
            black: IconUrls {
                floww: Url::of(&format!("{}tp/black/floww.png", base_url)),
                super_s: Url::of(&format!("{}tp/black/super_s.png", base_url)),
                super_t: Url::of(&format!("{}tp/black/super_t.png", base_url)),
                super_y: Url::of(&format!("{}tp/black/super_y.png", base_url)),
                ww: Url::of(&format!("{}tp/black/ww.png", base_url)),
                nft: Url::of(&format!("{}tp/black/yield_nft.png", base_url)),
            },
            white: IconUrls {
                floww: Url::of(&format!("{}tp/white/floww.png", base_url)),
                super_s: Url::of(&format!("{}tp/white/super_s.png", base_url)),
                super_t: Url::of(&format!("{}tp/white/super_t.png", base_url)),
                super_y: Url::of(&format!("{}tp/white/super_y.png", base_url)),
                ww: Url::of(&format!("{}tp/white/ww.png", base_url)),
                nft: Url::of(&format!("{}tp/white/yield_nft.png", base_url)),
            },
        };
        return set;
    }
}
