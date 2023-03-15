<h1 align="center" style="color:orange;"><b>Big O notation - Katta O belgisi</b></h1>


#### Big O notation nima va bu nima uchun muhim

"Big O notation - bu argument ma'lum bir qiymat yoki cheksizlikka moyil bo'lganda funktsiyaning cheklovchi xatti-harakatini tavsiflovchi matematik belgidir. Bu  Paul Bachmann, Edmund Landau va boshqalar tomonidan ixtiro qilingan notatsiyalar oilasining a'zosi bo'lib, birgalikda Bachmann-Landau notation yoki asymptotic notation deb ataladi.

— Vikipediyaning Big O notationning taʼrifi

Oddiy so'zlar bilan aytganda, Big O belgisi algebraik atamalar yordamida kodingizning murakkabligini tavsiflaydi.

Big O notation nima ekanligini tushunish uchun biz odatda "Big O squared(katta O kvadrat)" deb talaffuz qilinadigan O(n²) misolini ko'rib chiqishimiz mumkin.

Bu yerdagi “n” harfi input size(kirish hajmi)ni ifodalaydi va “O()” ichidagi “g(n) = n²” funksiyasi algoritmning kirish hajmiga nisbatan qanchalik murakkabligi haqida fikr beradi.

### Big O notatsiyasining rasmiy ta'rifi

Qadim zamonlarda bir hind podshohi bor ekan, u bir donishmandni o‘zining yuksakligi uchun mukofotlamoqchi edi. Donishmand shaxmat taxtasini to‘ldiradigan bug‘doydan boshqa hech narsa so‘ramaydi.

Ammo uning qoidalari quyidagicha edi: birinchi katakchada u 1 dona bug'doyni, keyin ikkinchi katakchada 2 dona, keyingisida 4 dona bug'doyni xohlaydi ... shaxmat taxtasidagi har bir katakchada avvalgisiga qaraganda ikki barobar ko'p don bilan to'ldirilishi kerak edi. Sodda podshoh ikkilanmasdan rozi bo'lib, buni amalga oshirish arzimas talab bo'ladi, deb o'yladi, toki u amalda davom etib, sinab ko'rmaguncha ...


Xo‘sh, shoh donishmanddan qancha bug‘doy qarzi bor? Biz bilamizki, shaxmat taxtasida 8 kvadratdan 8 kvadratga ega bo'lib, jami 64 ta katakcha bor. Shunday qilib, oxirgi katakcha jami 2⁶³ bug'doy doniga ega bo'lishi kerak. Agar siz onlayn hisob-kitob qilsangiz, butun shaxmat taxtasi uchun siz 1,8446744*10¹⁹ olasiz – bu taxminan 18 dan keyin 18 ta nolga teng.

Har bir bug'doy donining og'irligi 0,01 gramm, deb hisoblasak, bu bizga 184,467,440,737 tonna bug'doy beradi. Va 184 milliard tonna juda ko'p, shunday emasmi?

Raqamlar keyinchalik eksponensial o'sish uchun juda tez o'sadi, shunday emasmi? Xuddi shu mantiq kompyuter algoritmlari uchun ham amal qiladi. Agar topshiriqni bajarish uchun zarur bo'lgan harakatlar kirish hajmiga nisbatan eksponent ravishda o'ssa, u nihoyatda katta bo'lishi mumkin.

Birozdan keyin ko'rib turganimizdek, 2ⁿ ning o'sishi n² ga qaraganda tezroq. Endi, n = 64 bilan, 64 ning kvadrati 4096 ga teng. Agar bu raqamni 2⁶⁴ ga qo'shsangiz, u muhim raqamlardan tashqarida yo'qoladi.