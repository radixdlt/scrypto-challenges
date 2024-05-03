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
pub struct IconSet {
    pub transparent: IconColors,
    pub with_bg: IconColors,
}

#[derive(NonFungibleData, ScryptoSbor, Clone)]
pub struct IconColors {
    pub black: IconUrls,
    pub white: IconUrls,
}

impl IconSet {
    pub fn new() -> Self {
        let base_url: String = String::from("https://assets.floww.fi/images/logo/png/");

        let set: IconSet = IconSet {
            transparent: IconColors {
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
            },

            with_bg: IconColors {
                black: IconUrls {
                    floww: Url::of(&format!("{}bg/black/floww.png", base_url)),
                    super_s: Url::of(&format!("{}bg/black/super_s.png", base_url)),
                    super_t: Url::of(&format!("{}bg/black/super_t.png", base_url)),
                    super_y: Url::of(&format!("{}bg/black/super_y.png", base_url)),
                    ww: Url::of(&format!("{}bg/black/ww.png", base_url)),
                    nft: Url::of(&format!("{}bg/black/ww.png", base_url)),
                },
                white: IconUrls {
                    floww: Url::of(&format!("{}bg/white/floww.png", base_url)),
                    super_s: Url::of(&format!("{}bg/white/super_s.png", base_url)),
                    super_t: Url::of(&format!("{}bg/white/super_t.png", base_url)),
                    super_y: Url::of(&format!("{}bg/white/super_y.png", base_url)),
                    ww: Url::of(&format!("{}bg/white/ww.png", base_url)),
                    nft: Url::of(&format!("{}bg/white/yield_nft.png", base_url)),
                },
            },
        };

        return set;
    }
}
