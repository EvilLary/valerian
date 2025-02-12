<h1 align="center"> Valerian (ناردين) </h1>
<h4 align="center"> simple program to fetch cars from [TheCatAPI](https://thecatapi.com/) - بريمج صغير لجلب صور قطط</h4>
<h4 align="center"> It's a [ccat copycat](https://github.com/plastic-bottleneck/ccat) but way slower and more bloated :) &nbsp;<span dir="rtl">نفس ccat لكن ابطأ وأكبر</span> </h4>

![hola](assets/hola.jpg)

<h2 align="center"> <span dir="ltr">Installation </span>&nbsp;<span dir="rtl">تثبيت </span> </h2>




<h4 align="center"> On Arch and its derivatives - على ارش وأبنائه</h4>

1. create a temp directory - انشأ ملف للبناء

```bash
mkdir temp-build && cd temp-build
```
2. Download PKGBUILD script - حمل سكربت
- With curl - مع كيرل
```bash
curl https://codeberg.org/EvilLary/valerian/raw/branch/main/PKGBUILD > PKGBUILD
```
- Or wget - مع وجيت
```bash
wget https://codeberg.org/EvilLary/valerian/raw/branch/main/PKGBUILD
```

3. Build and install - ابني وثبت
```bash
makepkg -si
```

<h4 align="center"> Manually build it - ابنيه يدوي</h4>

```bash
git clone https://codeberg.org/EvilLary/Valerian.git
```
```bash
cd Valerian
```
```bash
cargo build --release
```
```bash
cp ./target/release/valerian $HOME/.local/bin/
```

<h2 align="center"> <span dir="ltr">Usage</span>&nbsp;<span dir="rtl">استعمال</span> </h2>


```bash
valerian -c <number-of-cars> -o <output-directory>
```
```bash
valerian -c 3 -o $HOME/Downloads/
```

<details>
<summary><h2>Breeds | العرق</h2></summary>
<br></br>

|           Breed Name          |Breed ID|
|               --              |   --   |
|  Abyssinian                   |  abys  |
|  Aegean                       |  aege  |
|  American Bobtail             |  abob  |
|  American Curl                |  acur  |
|  American Shorthair           |  asho  |
|  American Wirehair            |  awir  |
|  Arabian Mau                  |  amau  |
|  Australian Mist              |  amis  |
|  Balinese                     |  bali  |
|  Bambino                      |  bamb  |
|  Bengal                       |  beng  |
|  Birman                       |  birm  |
|  Bombay                       |  bomb  |
|  British Longhair             |  bslo  |
|  British Shorthair            |  bsho  |
|  Burmese                      |  bure  |
|  Burmilla                     |  buri  |
|  California Spangled          |  cspa  |
|  Chantilly-Tiffany            |  ctif  |
|  Chartreux                    |  char  |
|  Chausie                      |  chau  |
|  Cheetoh                      |  chee  |
|  Colorpoint Shorthair         |  csho  |
|  Cornish Rex                  |  crex  |
|  Cymric                       |  cymr  |
|  Cyprus                       |  cypr  |
|  Devon Rex                    |  drex  |
|  Donskoy                      |  dons  |
|  Dragon Li                    |  lihu  |
|  Egyptian Mau                 |  emau  |
|  European Burmese             |  ebur  |
|  Exotic Shorthair             |  esho  |
|  Havana Brown                 |  hbro  |
|  Himalayan                    |  hima  |
|  Japanese Bobtail             |  jbob  |
|  Javanese                     |  java  |
|  Khao Manee                   |  khao  |
|  Korat                        |  kora  |
|  Kurilian                     |  kuri  |
|  LaPerm                       |  lape  |
|  Maine Coon                   |  mcoo  |
|  Malayan                      |  mala  |
|  Manx                         |  manx  |
|  Munchkin                     |  munc  |
|  Nebelung                     |  nebe  |
|  Norwegian Forest Cat         |  norw  |
|  Ocicat                       |  ocic  |
|  Oriental                     |  orie  |
|  Persian                      |  pers  |
|  Pixie-bob                    |  pixi  |
|  Ragamuffin                   |  raga  |
|  Ragdoll                      |  ragd  |
|  Russian Blue                 |  rblu  |
|  Savannah                     |  sava  |
|  Scottish Fold                |  sfol  |
|  Selkirk Rex                  |  srex  |
|  Siamese                      |  siam  |
|  Siberian                     |  sibe  |
|  Singapura                    |  sing  |
|  Snowshoe                     |  snow  |
|  Somali                       |  soma  |
|  Sphynx                       |  sphy  |
|  Tonkinese                    |  tonk  |
|  Toyger                       |  toyg  |
|  Turkish Angora               |  tang  |
|  Turkish Van                  |  tvan  |
|  York Chocolate               |  ycho  |
</details>
