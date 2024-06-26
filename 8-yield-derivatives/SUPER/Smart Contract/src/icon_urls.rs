use scrypto::prelude::*;

#[derive(NonFungibleData, ScryptoSbor, Clone)]
pub struct IconUrls {
    pub floww: UncheckedUrl,
    pub super_s: UncheckedUrl,
    pub super_t: UncheckedUrl,
    pub super_y: UncheckedUrl,
    pub w: UncheckedUrl,
    pub nft: UncheckedUrl,
}

#[derive(NonFungibleData, ScryptoSbor, Clone)]
pub struct Icons {
    pub tp: IconUrls,
    pub bg: IconUrls,
}

impl Icons {
    pub fn new() -> Self {
        let base_url: String = String::from("https://assets.floww.fi/images/logo/png/");

        let set: Icons = Icons {
            tp: IconUrls {
                floww: Url::of(&format!("{}tp/floww.png", base_url)),
                super_s: Url::of(&format!("{}tp/super_s.png", base_url)),
                super_t: Url::of(&format!("{}tp/super_t.png", base_url)),
                super_y: Url::of(&format!("{}tp/super_y.png", base_url)),
                w: Url::of(&format!("{}tp/W.png", base_url)),
                nft: Url::of(&format!("{}tp/yield_nft.png", base_url)),
            },
            bg: IconUrls {
                floww: Url::of(&format!("{}bg/floww.png", base_url)),
                super_s: Url::of(&format!("{}bg/super_s.png", base_url)),
                super_t: Url::of(&format!("{}bg/super_t.png", base_url)),
                super_y: Url::of(&format!("{}bg/super_y.png", base_url)),
                w: Url::of(&format!("{}bg/W.png", base_url)),
                nft: Url::of(&format!("{}bg/yield_nft.png", base_url)),
            },
        };
        return set;
    }
}
