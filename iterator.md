# Iteratory

Biblioteka standardowa Rust zawiera [`trait Iterator`](https://doc.rust-lang.org/stable/core/iter/trait.Iterator.html) ([`trait`](https://doc.rust-lang.org/stable/book/ch10-02-traits.html) ≈ interfejs), która pozwala na iteracje przez różnego typu kolekcje, przedziały, itp.

Najważniejszą metodą iteratora jest `next`, która zwraca następny element (lub `None` jeśli został osiągnięty koniec iteracji). Na podstawie tej metody są zaimplementowane wszystkie bazujące na iteratorach funkcjonalności języka i biblioteki standardowej, n.p. pętle `for` i **adaptery iteratorów** ([*iterator adaptors*](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators)).

## Adaptery Iteratorów

Adaptery iteratorów są strukturami danych, które na podstawie jednego iteratora implementują inny. Przykładem takiego adaptera jest [`Map`](https://doc.rust-lang.org/stable/std/iter/struct.Map.html) zwracany przez metodę [`Iterator::map`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map), który został użyty w implementacji tego projektu.

Uproszczony przykład implementacji `Map`:

```rs
// Map jest generyczny i ma dwa parametry: iterator I na którym bazuje i
// funkcję F którą używa do przekształcenia elementów
// 
// (FnMut to kolejna trait automatycznie zaimplementowana dla wszystkich
// funkcji i domknięć, które mogą być wywołane wiele razy)
struct Map<I, F> {
	iter: I,
	f: F
}

// Implementacja trait Iterator dla struktury Map
impl<T, I: Iterator, F: FnMut(I::Item) -> T> Iterator for Map<I, F> {
	// Typ elementu iteratora jest ten sam, który zwraca funkcja F
	type Item = T;

	// Metoda next pobiera referencję do struktury na której została wywołana i
	// zwraca następny element iteracji
	fn next(&mut self) -> Option<Self::Item> {
		// Pobranie następnego elementu iteratora iter
		match self.iter.next() {
			// Jeśli został zwrócony element to zwrócony jest ten element po
			// przejściu przez funkcję f
		    Some(e) => Some((self.f)(e)),
			// Jeśli iterator iter się skończył, to metoda nic nie zwraca
		    None => None
		}
	}
}
```
