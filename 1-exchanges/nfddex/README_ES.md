# Non Fungible Data DEX

## El problema:

(Esta basado en hechos reales, hemos cambiado algunos nombres)
Una startup de desarrollo sostenible, ecológico y medioambiental, creada por dos Biólogos, desea ayudar a las empresas a alcanzar los objetivos 2030 (ODS) marcados por la Union Europea. Para ello desea mantener y crear ecosistemas generando excedentes de CO2 que luego desea vender permitiendo a sus clientes potenciales reducir su huella de carbono. A diferencia de otros proyectos parecidos, plantar arboles no será su prioridad, sino más bien, buscar el mantenimiento, conservación y regeneración de ecosistemas ya existentes. Para garantizar de forma feaciente su trabajo desean respaldar con certificados digitales los excedentes de CO2 a disposición y permitir en la medida de las posibilidades regulatorias el mercado de dichos excedentes. 

Actualmente la regulación Europea y Española (CNMV) reconocen las dificultades de encajar la creación "Utility Tokens" y "Security Tokens" dentro de la regulación actual del mercado de valores. 

Recientemente el proyecto español Potion.fi puso de manifiesto que los "no fungible tokens" de alguna forma bordean la regulación permitiendo utilizarlos de formas imaginativas. 

Aunque inicialmente la idea es crear, cual metaverso, una serie de "non fungible" relacionadas a un área geométrica de terreno y a su vez a una cantidad concreta de C02. La idea de que los usuarios puedan comerciar fácilmente con ellos sigue siendo fuerte. Para ello hemos estado realizando diversos prototipos de mercados. 

## Prototipo de intercambio de datos entre "non fungible"

Tres ideas hemos tenido en mente al pensar este prototipo inicial:
1. Permitir el comercio de C02
2. Simplificar los recursos, acuñando solo un "non fungible" por persona/empresa/institución.
3. Cumplir tanto conceptualmente como técnicamente los objetivos 2030 de la UE.

### Caracteristicas del Prototipo

1. Emitir "non fungible" que contenga un campo de tipo numérico para guardar el C02 comprado. (Todos)
2. Incluir liquidez de C02 (Admin)
3. Cambiar el ratio de venta de C02 con respecto a XRD, se iniciara con ratio 1:1 (Admin)
4. Comprar C02 a cambio de XRD teniendo en cuenta el ratio establecido y agregando dato en nuestro "non fungible". (Todos)
5. Permitir vender C02 a los usuarios a cambio de XRD teniendo en cuenta el ratio establecido y restando el dato al "non fungible" (Todos)

No hemos tenido en cuenta, comisiones, caducidad de la compra, relación del C02 con el lugar donde se consume, etc...

Pensamientos: La idea de un solo "non fungible" y no multiples viene dada por dos conceptos:
1. Economizar y simplificar la interacción del usuario. (proyecto + ecológico)
2. Relacionar la futura SSI con la reducción de CO2 que posee.

## Getting Started
0. Limpiar simulador:
```
resim reset
```
1. Crear una cuenta:
```
resim new-account
```
2. Guardar dirección
```
set acct <address>
```
3. Publicar package
```
resim publish .
```
4. Guardar package address
```
set pack <address>
```
5. Instanciar componente
```
resim call-function $pack Ddex new
```
6. Guardar dirección del componente, xrd, admin badge
```
set comp <address>
set xrd 030000000000000000000000000000000000000000000000000004
set admin <resource_def>
```
7. Aportar liquidez de "Data"
```
resim call-method $comp data_transfer 10000 1,$admin
```
8. Acuñar un Nft
```
resim call-method $comp mint
```
9. Comprar "Data"
```
resim call-method $comp buy 100,$xrd 1,<nft def>
```
10. Vender "Data"
```
resim call-method $comp sell 50,$xrd 1,<nft def>
```
### Cambiar ratio
```
resim call-method $comp ratio 10000 1,$admin
```

### Transaction Manifest
CALL_METHOD Address("<component address>") "mint";
TAKE_FROM_WORKTOP Decimal("1") Address("<nft reference>") Bucket("NFT");

CALL_METHOD Address("<my account address>") "withdraw" Decimal("100") Address("030000000000000000000000000000000000000000000000000004") BucketRef(1u32);

TAKE_FROM_WORKTOP Decimal("100") Address("030000000000000000000000000000000000000000000000000004") Bucket("XRD");

CREATE_BUCKET_REF Bucket("NFT") BucketRef("NFT_ref");
CALL_METHOD Address("<component address>") "buy" Bucket("XRD") BucketRef("NFT_ref");

CALL_METHOD_WITH_ALL_RESOURCES Address("<my account address>") "deposit_batch";










