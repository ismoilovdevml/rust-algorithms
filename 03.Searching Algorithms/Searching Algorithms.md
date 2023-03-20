<h1 align="center" style="color:orange;"><b>Searching Algorithms - Qidiruv Algoritmlari</b></h1>

Computer scienceda search(qidiruv) algoritmi qidiruv masalasini yechish uchun mo‘ljallangan algoritmdir. Search algoritmlari ma'lum bir ma'lumotlar strukturasida saqlangan yoki muammoli domenning qidiruv maydonida hisoblangan ma'lumotlarni diskret yoki doimiy qiymatlar bilan olish uchun ishlaydi. Qidiruv tizimlari search algoritmlaridan foydalanishiga qaramasdan, ular algoritmik emas, balki ma'lumotni qidirishni o'rganishga tegishli. Tegishli qidiruv algoritmi ko'pincha qidirilayotgan ma'lumotlar strukturasiga bog'liq va ma'lumotlar haqida oldingi bilimlarni ham o'z ichiga olishi mumkin.

Qidiruv algoritmlari search trees, hash maplar va database indekslari kabi maxsus tuzilgan database tuzilmalari orqali tezroq yoki samaraliroq bo'lishi mumkin. Qidiruv algoritmlarini qidirish mexanizmiga ko'ra uchta algoritm turiga bo'lish mumkin: linear, binary va hashing(xeshlash).

## Seach Algoritmlar turlari

### Linear Search

Linear Search, shuningdek, ketma-ket qidiruv sifatida ham tanilgan, eng oddiy qidiruv algoritmlaridan biridir. Ko'pincha elementlar ro'yxati tartiblanmagan yoki ro'yxat kichik bo'lsa ishlatiladi. Eng yomon holatda, linear search O (n) time complexityga(vaqt murakkabligi) ega, bu yerda n - ro'yxatdagi elementlar soni.

### Binary Search

Binary Search - saralangan ro'yxatlarda ishlaydigan yanada samarali qidiruv algoritmi. Maqsadli element topilmaguncha yoki qidiruv oralig'i bo'sh bo'lgunga qadar qidiruv oralig'ini qayta-qayta ikkiga bo'ladi. Binary Search O(log n) time complexityga ega, bu yerda n - ro'yxatdagi elementlar soni. Bu uni linear searchdan ko'ra tezroq qiladi, ayniqsa kattaroq ro'yxatlar uchun.

### Depth-First Search

Depth-First Search - bu orqaga qaytishdan oldin har bir filial bo'ylab imkon qadar uzoqroqqa o'rganadigan graph traversal(grafik o'tish) algoritmi. U ko'pincha graphdagi ma'lum bir nodeni(tugunni) qidirish yoki butun graphni aylanib o'tish uchun ishlatiladi. Depth-first search time complexitygi O(V + E), bu yerda V - number vertice va E - graphdagi number edge.

### Breadth-First Search

Breadth-First Search - bu keyingi depthdagi nodelarga o'tishdan oldin hozirgi depthkdagi barcha qo'shni nodelarni o'rganadigan yana graph traversal algoritmi. U ko'pincha graphdagi ma'lum bir nodeni qidirish yoki butun graphni aylanib o'tish uchun ishlatiladi. Breadth-First Search ham O (V + E) time complexityga ega.

### A* Search

A* Search evristik qidiruv algoritmi boʻlib, u koʻpincha yoʻlni topish va graph traversalda qoʻllaniladi. U joriy nodedan maqsad nodegacha bo'lgan optimal yo'lning narxini hisoblaydigan evristik baholash funksiyasi bilan depth-first search  va breadth-first search algoritmlarini birlashtiradi. A* search O(b^d) time complexityga ega, bunda b - grafikning tarmoqlanish omili va d - optimal yechimning chuqurligi.

### Dijkstra Algorithm

Dijkstra Algorithm boshqa mashhur search algoritmi bo'lib, u ko'pincha yo'lni topish va graph traversalda qo'llaniladi. U A* searchga o'xshaydi, lekin evristik baholash funktsiyasidan foydalanmaydi. Buning o'rniga, u boshlang'ich nodegacha bo'lgan masofaga qarab tashrif buyuradigan keyingi nodeni kuzatib borish uchun ustuvor navbatdan foydalanadi. Dijkstra Algoritmi O((V+E) log V) time complexityga ega.


Bu search algoritmlarining bir nechta misollari. Boshqa ko'plab search algoritmlari mavjud bo'lib, ularning har biri muammo sohasiga qarab o'zining kuchli va zaif tomonlariga ega. Asosan Linear Search va Binary Search algoritmlari ishlatiladi. Linear Search tartibsiz ro'yxatlar uchun, Binary Search esa tartiblangan ro'yxatlar uchun ishlatiladi.