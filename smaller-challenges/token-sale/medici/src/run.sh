run.sh - Bash
#!/bin/bash
resim reset

echo "Creating account"
resim new-account
acc=020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
pubkey=044083a64afb4b630ce7683674a6cdcebc7007aef7cb08f10b2cd491b6ce24ca1204f88bd2a2068e27591f1c5cfbd4fddf9a51f7b2360d784ee1e8fbec8f7476a6
privk=7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8
echo ""

echo "Publishing package"
pkg=011bfdc7c7240e386a07bfa67b9e16dae6da874e2ca0046b24a29f
resim publish . --package-address $pkg
echo ""

echo "Creating token"
resim call-function $pkg TokenSale new 3
xrd=030000000000000000000000000000000000000000000000000004
tokensaleBlueprint=0266a286e6d6295632f24e0afd11c84e104da8bcb04ed1392c7109
tokensale=0304f6a5e8bfba20a39191ff52b9c494911b9fa9895de72fc9254d
seller_badge=036714bd2c97555eb4bbb477fa75070092f1b6570e79e87b13138d

echo "Buying 100 tokens with XRD"
resim call-method $tokensaleBlueprint buy 100,$xrd
echo ""

echo "Trying to Buying 100 tokens with my tokens"
resim call-method $tokensaleBlueprint buy 10,$tokensale
# Nice!: Transaction Status: VaultError(ResourceContainerError(ResourceAddressNotMatching))
echo ""

echo "Show buyer account"
resim show $acc
echo ""

echo "Show seller vault"
resim show $tokensaleBlueprint

echo "Withdrawing funds"
resim new-account
admin=021b1871540abf576a9901994265ca5a46364531c272cc09616c8b
admin_pubkey=0437a82e8f7cd28e29f1cba48905050a2de9e999c2466add4a1d83a7783324a26cc20fff53531c02421039fa4098bd8b5533643e1d274b46d6b7a54f04e013155f
admin_k=23d7f42b1cdc1f0d492ebd756ed0fe8003995dda554d99418d47a81813650207

echo "Trying to withdraw as a user"
resim call-method $tokensaleBlueprint withdraw_funds 50