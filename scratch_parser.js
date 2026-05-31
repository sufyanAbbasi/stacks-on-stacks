const fs = require('fs');
const path = require('path');
const { parseCard } = require('./vendor/magic-card-parser/src/magicCardParser');

const cardNames = [
    "Thraben Doomsayer",
    "Colossal Dreadmaw",
    "Sol Ring",
    "Runeclaw Bear",
    "Giant Growth",
    "Behold the Multiverse",
    "Slickshot Show-Off",
    "Giant Beaver",
    "Dawn's Truce",
    "Abhorrent Oculus",
    "Case of the Filched Falcon",
    "Adrix and Nev, Twincasters",
    "Ranger Class",
    "Future Sight",
    "Courser of Kruphix"
];

const customCards = [
    {
        name: "Emeritus of Ideation",
        oracle_text: "Emeritus of Ideation enters the battlefield prepared.",
        layout: "normal"
    },
    {
        name: "Abigale",
        oracle_text: "Whenever you cast a creature spell, Abigale becomes prepared.",
        layout: "normal"
    },
    {
        name: "Saddle Beaver",
        oracle_text: "Saddle 3",
        layout: "normal"
    },
    {
        name: "Adrix Ward",
        oracle_text: "Ward {2}",
        layout: "normal"
    },
    {
        name: "Dawn Gift",
        oracle_text: "Gift a card",
        layout: "normal"
    },
    {
        name: "Foretell Multiverse",
        oracle_text: "Foretell {1}{U}",
        layout: "normal"
    },
    {
        name: "Slickshot Plot",
        oracle_text: "Plot {1}{R}",
        layout: "normal"
    },
    {
        name: "Evidence Collector",
        oracle_text: "Collect evidence 6",
        layout: "normal"
    },
    {
        name: "Discoverer",
        oracle_text: "Discover 4.",
        layout: "normal"
    }
];

const CACHE_DIR = path.join(__dirname, 'scratch', 'scryfall_cache');

async function fetchCard(name) {
    if (!fs.existsSync(CACHE_DIR)) {
        fs.mkdirSync(CACHE_DIR, { recursive: true });
    }
    const safeName = name.replace(/[^a-zA-Z0-9]/g, '_').toLowerCase();
    const cachePath = path.join(CACHE_DIR, `${safeName}.json`);

    if (fs.existsSync(cachePath)) {
        try {
            return JSON.parse(fs.readFileSync(cachePath, 'utf8'));
        } catch (e) {
            console.warn(`[CACHE ERROR] Failed to parse cached file for ${name}, refetching...`);
        }
    }

    const url = `https://api.scryfall.com/cards/named?exact=${encodeURIComponent(name)}`;
    console.log(`Fetching from Scryfall: ${name}...`);
    try {
        // Simple delay before fetching to avoid rate limits
        await new Promise(resolve => setTimeout(resolve, 100));
        
        const response = await fetch(url, {
            headers: {
                'User-Agent': 'stacks-on-stacks/0.1.0'
            }
        });
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        fs.writeFileSync(cachePath, JSON.stringify(data, null, 2), 'utf8');
        return data;
    } catch (e) {
        console.error(`Failed to fetch card ${name}:`, e.message);
        return null;
    }
}

async function main() {
    const results = [];
    
    // 1. Fetch and parse standard cards
    for (const name of cardNames) {
        const card = await fetchCard(name);
        if (!card) continue;

        const inputCard = {
            name: card.name,
            oracle_text: card.oracle_text,
            layout: card.layout || 'normal'
        };

        console.log(`Parsing card: ${card.name}...`);
        const parsed = parseCard(inputCard);
        
        if (parsed.error) {
            console.warn(`⚠️ Warning: Parser returned error for ${card.name}:`, parsed.error);
        } else {
            console.log(`✅ Success parsing: ${card.name}`);
        }
        
        results.push({
            name: card.name,
            oracle_text: card.oracle_text,
            parse_result: parsed
        });
    }

    // 2. Parse mock/custom cards
    for (const card of customCards) {
        console.log(`Parsing custom card: ${card.name}...`);
        const parsed = parseCard(card);
        
        if (parsed.error) {
            console.warn(`⚠️ Warning: Parser returned error for ${card.name}:`, parsed.error);
        } else {
            console.log(`✅ Success parsing: ${card.name}`);
        }
        
        results.push({
            name: card.name,
            oracle_text: card.oracle_text,
            parse_result: parsed
        });
    }

    const outputPath = path.join(__dirname, 'parsed_cards.json');
    fs.writeFileSync(outputPath, JSON.stringify(results, null, 2), 'utf-8');
    console.log(`\nAll done! Results saved to: ${outputPath}`);
}

main();
