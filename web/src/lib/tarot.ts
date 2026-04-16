import { getCollection, type CollectionEntry } from "astro:content";

export async function listTarotCards() {
  const entries = await getCollection("tarot");
  return entries.sort((left, right) => left.data.order - right.data.order);
}

export async function getTarotCard(slug: string) {
  const cards = await listTarotCards();
  return cards.find((card) => card.id === slug) ?? null;
}

export function getTarotCardHref(card: Pick<CollectionEntry<"tarot">, "id">) {
  return `/cards/${card.id}`;
}
