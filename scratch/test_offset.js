const text = "gift a card (you may promise an opponent a gift as you cast this spell. if you do, they draw a card before its other effects.)\nyou and permanents you control gain hexproof until end of turn. if the gift was promised, permanents you control also gain indestructible until end of turn.";
for (let i = 150; i < 180; i++) {
    console.log(`${i}: ${JSON.stringify(text[i])}`);
}
