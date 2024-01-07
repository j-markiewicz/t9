# `multitap()` - **`O(ilość słów w słowniku * długość wejścia)`**

Funkcja `multitap` zwraca trzy najlepiej pasujące słowa dla podanago wejścia w podanym języku dla metody wejścia [*multi-tap*](https://en.wikipedia.org/wiki/Multi-tap).
Pierwsze ze zwróconych słów jest niezmodyfikowaną wersją wejścia (jeśli wejście to `2229999337777722222` dla języka polskiego, to pierwsze słowo jest `cześć`).
Drugie i trzecie zwrócone słowa to najbardziej popularne słowa zaczynające się na w.w. słowo wybrane ze słownika.

Pełna funkcja znajduje się w pliku `src/lib.rs`. `multitap` działa w następujący sposób:

- ```rs
  #[instrument]
  pub fn multitap(input: &[Character], lang: Language) -> [Cow<'static, str>; 3] {
  	...
  }
  ```

  Definicja funkcji oraz instrumentacja, dzięki której wypisywane są informacje o wywołaniach funkcji

- ```rs
  if input.is_empty() {
  	return [
  		Cow::Borrowed(""),
  		Cow::Borrowed(":-)"),
  		Cow::Borrowed(":-("),
  	];
  }
  ```

  Dla pustego wejścia zwrócone są powyższe wartości, aby ekran aplikacji nie był pusty - **`O(1)`**

- ```rs
  let mut buf = String::new();
  let mut current = (None, 0);

  for &char in input {
  	if current.0 == Some(char) {
  		current.1 += 1;
  	} else {
  		if let Some(cur) = current.0 {
  			let chars = cur.chars(lang);
  			buf.push(chars[current.1 % chars.len()]);
  		}
  		current.1 = 0;
  		current.0 = Some(char);
  	}
  }

  if let Some(cur) = current.0 {
  	let chars = cur.chars(lang);
  	buf.push(chars[current.1 % chars.len()]);
  }
  ```

  Szukanie odpowiedniego znaku dla podanego wejścia - **`O(długość wejścia)`**

  `buf` zawiera ciąg znaków, który jest generowany przez pętlę, `current` zawiera aktualny znak wejścia oraz liczbę jego kolejnych wystąpień

  Po każdym napotkaniu się na nowy znak wejścia, litera odpowiadająca poprzedniemu ciągu znaków jest wstawiona na koniec `buf` i rozpoczyna się nowe liczenie znaków

  Po zakończeniu pętli ostatni znak jest dodany do `buf`

  Ta pętla znajduje się w osobnej funkcji (`multitap_decode`), ponieważ jest również używana w niektórych przypadkach przez funkcję `t9`

- ```rs
  let mut words = match lang {
  	Language::En => WORDS_EN,
  	Language::Pl => WORDS_PL,
  }
  .lines()
  .filter(|w| w.starts_with(&buf) && w != &buf);

  let sug = (
  	words.next().unwrap_or_default(),
  	words.next().unwrap_or_default(),
  );
  ```

  Wyszukanie odpowiednich słów z odpowiedniego słownika - **`O(ilość słów w słowniku * długość buf)`** (długość `buf` jest równa długości wejścia z usuniętymi wszystkimi sąsiadującymi powtórzeniami, więc jest mniejsza lub równa długości wejścia)

  Stworzony jest iterator, który będzie przechodził po liniach odpowiedniego słownika, zwracając tylko te słowa, które zaczynają się na wpisane słowo i nie są mu równe

  Samo stworzenie iteratora jest bardzo szybkie, ponieważ polega tylko na inicjalizacji pewnych struktur danych, i nie są wykonywane żadne iteracje

  Później metoda `.next()` jest wywołana na iteratorze dwa razy aby znaleźć odpowiednie słowa, co powoduje wykonanie wystarczającej ilości iteracji (między 2 a ilości słów w słowniku)

  Jeśli `.next()` niczego nie zwróci, to znaczy że nie ma pasującego słowa w słowniku i użyty jest pusty ciąg znaków

- ```rs
  [Cow::Owned(buf), Cow::Borrowed(sug.0), Cow::Borrowed(sug.1)]
  ```
  
  Zwrócenie wyników - **`O(1)`**

  Wyniki są zwrócone w kontenerze `Cow` (*copy-on-write*), aby równocześnie móc stworzyć i zwrócić nowe ciągi znaków (tutaj `buf`) oraz ciągi znaków wcześniej zaalokowane (tutaj `sug.0` i `sug.1`, które są referencjami do części słownika)
