const { parseCard } = require('./vendor/magic-card-parser/src/magicCardParser');

// Read input from stdin
let inputData = '';
process.stdin.on('data', chunk => {
    inputData += chunk;
});

process.stdin.on('end', () => {
    try {
        if (!inputData.trim()) {
            console.error(JSON.stringify({ error: "Empty input" }));
            process.exit(1);
        }
        const card = JSON.parse(inputData);
        const parsed = parseCard(card);
        console.log(JSON.stringify(parsed));
    } catch (e) {
        console.error(JSON.stringify({ error: e.message }));
        process.exit(1);
    }
});
