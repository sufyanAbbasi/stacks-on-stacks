import re
import os

# Helper to determine esoteric category
def get_esoteric_category(rnum):
    if rnum.startswith("123.") or rnum.startswith("717."):
        return "ESOTERIC_CARD_ATTRIBUTES"
    if any(
        rnum.startswith(prefix)
        for prefix in ["311.", "312.", "313.", "314.", "315."]
    ):
        return "ESOTERIC_CARD_TYPES"
    if rnum.startswith("116.2i") or rnum.startswith("116.2j"):
        return "ESOTERIC_SPECIAL_ACTIONS"
    if any(
        rnum.startswith(prefix)
        for prefix in ["719.", "720.", "721.", "722.", "728."]
    ):
        return "ESOTERIC_MECHANICS"
    if any(rnum.startswith(prefix) for prefix in ["807.", "809.", "811."]):
        return "ESOTERIC_MULTIPLAYER_VARIANTS"
    if any(rnum.startswith(prefix) for prefix in ["901.", "902.", "904.", "905."]):
        return "ESOTERIC_CASUAL_VARIANTS"
    return None

# Noise words
noise_words = {
    # Pronouns & Demonstratives
    "A", "AN", "THE", "IT", "ITS", "THEY", "THEM", "THEIR", "THEIRS", "THEYRE",
    "HE", "SHE", "HIM", "HER", "HIS", "HIMSELF", "HERSELF", "MY", "MINE", "ME",
    "WE", "US", "OUR", "OURS", "YOU", "YOUR", "YOURS", "THAT", "THIS", "THESE",
    "THOSE", "WHO", "WHOM", "WHOSE", "WHICH", "WHAT", "WHATEVER", "WHOEVER",
    # Prepositions
    "OF", "TO", "IN", "ON", "AT", "BY", "FOR", "WITH", "FROM", "ABOUT",
    "AGAINST", "BETWEEN", "INTO", "THROUGH", "DURING", "BEFORE", "AFTER",
    "ABOVE", "BELOW", "UP", "DOWN", "OFF", "OVER", "UNDER", "AGAIN", "AMONG",
    "THROUGHOUT", "UPON", "WITHIN", "WITHOUT", "UNTIL", "SINCE",
    # Conjunctions & Transitions
    "AND", "OR", "BUT", "SO", "YET", "NOR", "ALTHOUGH", "THOUGH", "BECAUSE",
    "UNLESS", "IF", "WHEN", "WHENEVER", "WHEREAS", "WHETHER", "WHILE", "AS",
    "ALSO", "THEN", "THUS", "THEREFORE", "HOWEVER", "BESIDES", "FURTHERMORE",
    "MOREOVER", "INSTEAD",
    # Auxiliary & Linking Verbs
    "IS", "ARE", "WAS", "WERE", "BE", "BEING", "BEEN", "CAN", "COULD", "WOULD",
    "SHOULD", "MAY", "MIGHT", "MUST", "WILL", "SHALL", "DO", "DOES", "DID",
    "DONE", "DOING", "HAS", "HAVE", "HAD", "HAVING",
    # Reference & Procedural Verbs (dangling or introductory)
    "SEE", "SEES", "SAW", "SEEN", "REFER", "REFERS", "REFERRED", "REFERRING",
    "MEAN", "MEANS", "MEANT", "MEANING", "DETERMINE", "DETERMINES", "DETERMINED",
    "DETERMINING", "FOLLOW", "FOLLOWS", "FOLLOWED", "FOLLOWING", "INSTRUCT",
    "INSTRUCTS", "INSTRUCTED", "INSTRUCTING", "USE", "USES", "USED", "USING",
    "FIND", "FINDS", "FOUND", "FINDING", "GIVE", "GIVES", "GIVEN", "GIVING",
    "TAKE", "TAKES", "TAKEN", "TAKING", "MAKE", "MAKES", "MADE", "MAKING",
    "GET", "GETS", "GOT", "GETTING", "GO", "GOES", "WENT", "GONE", "GOING",
    "COME", "COMES", "CAME", "COMING", "CALL", "CALLS", "CALLED", "CALLING",
    "DESCRIBE", "DESCRIBES", "DESCRIBED", "DESCRIBING", "EXPLAIN", "EXPLAINS",
    "EXPLAINED", "EXPLAINING", "KNOW", "KNOWS", "KNOWN", "KNOWING", "REPRESENT",
    "REPRESENTS", "REPRESENTED", "REPRESENTING",
    # General noise / reference text
    "RULE", "RULES", "SUBRULE", "SUBRULES", "SECTION", "SECTIONS", "CHAPTER",
    "CHAPTERS", "PART", "PARTS", "MORE", "LESS", "MOST", "LEAST", "SOME",
    "ANY", "EACH", "EVERY", "ALL", "BOTH", "OTHER", "OTHERS", "ANOTHER",
    "SUCH", "CERTAIN", "PARTICULAR", "SPECIFIC", "VARIOUS", "DIFFERENT",
    "SAME", "INFORMATION", "EXAMPLE", "EXAMPLES", "NOTE", "NOTES", "PLEASE",
    "WIZARDS", "WIZARDSCOM", "GATHERER", "ORACLE", "DATABASE", "WPNWIZARDSCOM",
    # Numbers (often noise in titles)
    "ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE", "TEN",
    "FIRST", "SECOND", "THIRD", "FOURTH", "FIFTH",
    # Common adverbs & Filler words
    "STILL", "ONLY", "JUST", "ALREADY", "NOT", "NO", "NEVER", "ALWAYS", "VERY",
    "TOO", "EVEN", "WELL", "BACK", "AGAIN", "TOGETHER", "SEPARATELY", "USUALLY",
    "NORMALLY", "GENERALLY", "THERE", "HERE",
    # Contraction residues / single letter noise
    "S", "T", "RE", "D", "M", "LL", "VE",
}

condition_keywords = {"if", "when", "whenever", "unless", "at", "instead", "would"}

def make_refined_name(primary_rnum, text):
    # Strip reference parentheticals like "(see rule 103...)" or "(see section...)"
    text_no_ref = re.sub(r"\(\s*[sS]ee\s+rule\s+[^)]+\)", "", text)
    text_no_ref = re.sub(r"\(\s*[sS]ee\s+section\s+[^)]+\)", "", text_no_ref)
    
    # Remove apostrophes completely so they don't become space-separated letters (e.g. they're -> theyre)
    text_no_apostrophes = text_no_ref.replace("'", "").replace("’", "")
    
    # Clean text: remove special characters, make uppercase
    text_clean = re.sub(r"[^A-Z0-9\s]", " ", text_no_apostrophes.upper())
    words = text_clean.split()
    
    # Filter out noise words, numeric literals, and duplicate/stem-duplicate words
    filtered_words = []
    seen_stems = set()
    for w in words:
        if w in noise_words or w.isdigit():
            continue
        
        # Simple stemming to prevent duplicate-stem adjacencies (e.g., COLOR_COLOR, COLOR_COLORS, TEAM_TEAM)
        stem = w
        if len(w) > 3:
            if w.endswith("IES"):
                stem = w[:-3] + "Y"
            elif (
                w.endswith("ES")
                and not w.endswith("SS")
                and any(w[:-2].endswith(sfx) for sfx in ("CH", "SH", "X", "S", "Z"))
            ):
                stem = w[:-2]
            elif w.endswith("S") and not w.endswith("SS"):
                stem = w[:-1]
        
        if stem in seen_stems:
            continue
            
        seen_stems.add(stem)
        filtered_words.append(w)
    
    # Let's limit to 5 words to keep it compact and compile-friendly
    selected_words = filtered_words[:5]
    
    desc = "_".join(selected_words)
    rnum_clean = primary_rnum.replace(".", "_")
    if desc:
        return f"RULE_{rnum_clean}_{desc}"
    else:
        return f"RULE_{rnum_clean}"

def check_cond_heuristic(text):
    text_clean = re.sub(r"[^a-zA-Z\s]", " ", text.lower())
    words = set(text_clean.split())
    return not words.isdisjoint(condition_keywords)

OVERRIDE_NAMES = {
    "105.1": "RULE_105_1_FIVE_COLORS_IN_MAGIC",
    "105.2": "RULE_105_2_OBJECT_COLORS_OR_COLORLESS",
    "105.2a": "RULE_105_2a_MONOCOLORED_OBJECT",
    "105.2b": "RULE_105_2b_MULTICOLORED_OBJECT",
    "105.2c": "RULE_105_2c_COLORLESS_OBJECT",
    "201.3": "RULE_201_3_INTERCHANGEABLE_NAMES_SAME_NAME",
    "207.2": "RULE_207_2_ITALICIZED_TEXT_HAS_NO_GAME_FUNCTION",
    "208.3": "RULE_208_3_NONCREATURE_PERMANENT_HAS_NO_POWER_TOUGHNESS",
    "301.5": "RULE_301_5_EQUIPMENT_CONTROL_AND_ATTACHMENT",
    "507.1": "RULE_507_1_MULTIPLAYER_CHOOSE_DEFENDING_PLAYER",
    "701.49": "RULE_701_49_VENTURE_INTO_DUNGEON",
    "702.26": "RULE_702_26_PHASING_MECHANICS",
    "726.2": "RULE_726_2_INITIATIVE_INHERENT_TRIGGERED_ABILITIES",
    "805.10": "RULE_805_10_SHARED_TEAM_COMBAT_BLOCK_DAMAGE",
    "903.5": "RULE_903_5_COMMANDER_DECK_CONSTRUCTION_RESTRICTIONS",
}

def main():
    rules_text_path = "./rules/MagicCompRules_20260417.txt"
    rules_rs_path = "./src/compiler/rules.rs"

    # 1. Parse MagicCompRules_20260417.txt
    rule_pattern = re.compile(r"^(\d{3}\.\d+[a-z]?)\.?\s+(.*)$")
    all_rules = []  # list of tuples: (rule_num, text)
    comp_rules = {}  # dict: rule_num -> text

    with open(rules_text_path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            if line.startswith("Example:"):
                continue
            match = rule_pattern.match(line)
            if match:
                rule_num = match.group(1)
                rule_text = match.group(2)
                if rule_num not in comp_rules:
                    comp_rules[rule_num] = rule_text
                    all_rules.append((rule_num, rule_text))

    # 2. Parse existing src/compiler/rules.rs to find already mapped rule numbers and variants
    with open(rules_rs_path, "r", encoding="utf-8") as f:
        rs_lines = f.readlines()

    current_comments = []
    existing_rule_to_variant = {}  # rnum -> (variant_name, has_condition)
    existing_variants = set()

    for line in rs_lines:
        stripped = line.strip()
        if stripped.startswith("//"):
            current_comments.append(stripped)
        elif stripped:
            match = re.match(r"^\s*([a-zA-Z0-9_]+)(\(Condition\))?,?\s*$", line)
            if match:
                variant_name = match.group(1)
                has_cond_in_file = match.group(2) is not None
                existing_variants.add(variant_name)
                for comment in current_comments:
                    m_rnum = re.match(r"^\s*//\s*(\d{3}\.\d+[a-z]?)\.?\s+", comment)
                    if m_rnum:
                        rnum = m_rnum.group(1)
                        primary_m = re.match(r"^(\d{3}_\d+)", rnum.replace(".", "_"))
                        if primary_m:
                            expected_prefix = "RULE_" + primary_m.group(1)
                            # Check if matches expected prefix or is the correct esoteric placeholder
                            is_esoteric_match = variant_name.startswith("ESOTERIC_") and get_esoteric_category(rnum) == variant_name
                            is_rule_match = variant_name.startswith(expected_prefix)
                            if is_rule_match or is_esoteric_match:
                                # Exclude 601.2a-i from being mapped to RULE_601_2_CAST_SPELL_WHERE_HAND_PUT
                                if rnum.startswith("601.2") and rnum != "601.2" and variant_name == "RULE_601_2_CAST_SPELL_WHERE_HAND_PUT":
                                    continue
                                existing_rule_to_variant[rnum] = (variant_name, has_cond_in_file)
                current_comments = []
            else:
                if "enum Rule" not in line and "struct Condition" not in line:
                    current_comments = []

    print(f"Parsed {len(existing_rule_to_variant)} existing rule mappings from src/compiler/rules.rs.")

    # 3. Group and assign all rules from MagicCompRules_20260417.txt
    used_variant_names = set(existing_variants)
    generated_primary_to_variant = {}
    
    esoteric_placeholders_list = [
        "ESOTERIC_CARD_ATTRIBUTES",
        "ESOTERIC_CARD_TYPES",
        "ESOTERIC_SPECIAL_ACTIONS",
        "ESOTERIC_MECHANICS",
        "ESOTERIC_MULTIPLAYER_VARIANTS",
        "ESOTERIC_CASUAL_VARIANTS"
    ]
    
    group_map = {}
    ordered_groups = []
    
    # Pre-populate esoteric placeholders to preserve them
    for ph in esoteric_placeholders_list:
        group_dict = {
            "variant_name": ph,
            "has_condition": False,
            "is_esoteric": True,
            "rules": []
        }
        group_map[ph] = group_dict
        ordered_groups.append(group_dict)

    for rnum, text in all_rules:
        if rnum in existing_rule_to_variant:
            var_name, has_cond = existing_rule_to_variant[rnum]
            is_esoteric = var_name.startswith("ESOTERIC_")
        else:
            eso_cat = get_esoteric_category(rnum)
            if eso_cat:
                var_name = eso_cat
                has_cond = False
                is_esoteric = True
            else:
                is_esoteric = False
                if (
                    rnum.startswith("105.2")
                    or rnum.startswith("107.4")
                    or rnum.startswith("201.2")
                    or rnum.startswith("202.2")
                    or rnum.startswith("205.1")
                    or rnum.startswith("601.2")
                ):
                    primary = rnum
                else:
                    m = re.match(r"^(\d{3}\.\d+)", rnum)
                    if m:
                        primary = m.group(1)
                    else:
                        primary = rnum
                
                if primary in OVERRIDE_NAMES:
                    var_name = OVERRIDE_NAMES[primary]
                else:
                    if primary not in generated_primary_to_variant:
                        candidate_name = make_refined_name(primary, text)
                        var_name = candidate_name
                        suffix = 2
                        while var_name in used_variant_names:
                            var_name = f"{candidate_name}_{suffix}"
                            suffix += 1
                        used_variant_names.add(var_name)
                        generated_primary_to_variant[primary] = var_name
                    var_name = generated_primary_to_variant[primary]
                
                has_cond = check_cond_heuristic(text)

        # Add to group
        if var_name not in group_map:
            group_dict = {
                "variant_name": var_name,
                "has_condition": has_cond,
                "is_esoteric": is_esoteric,
                "rules": []
            }
            group_map[var_name] = group_dict
            ordered_groups.append(group_dict)
        else:
            group_dict = group_map[var_name]
            if has_cond:
                group_dict["has_condition"] = True
                
        group_dict["rules"].append((rnum, text))

    # Find the header of the file (everything up to enum Rule {)
    header_lines = []
    enum_found = False
    for line in rs_lines:
        if "enum Rule" in line:
            enum_found = True
            break
        header_lines.append(line)
        
    if not enum_found:
        raise Exception("Could not find 'enum Rule' in src/compiler/rules.rs!")

    output_lines = []
    output_lines.extend(header_lines)
    output_lines.append("enum Rule {\n")

    # Write Esoteric Placeholders
    output_lines.append("\n")
    output_lines.append("    // =========================================================================\n")
    output_lines.append("    // GENERATED CORE AND ESOTERIC RULES (CHAPTERS 1-9)\n")
    output_lines.append("    // =========================================================================\n\n")
    output_lines.append("    // --- ESOTERIC AND CASUAL PLAY VARIANTS PLACEHOLDERS ---\n\n")
    
    for ph in esoteric_placeholders_list:
        group_dict = group_map[ph]
        rules_list = group_dict["rules"]
        output_lines.append(f"    // Placeholder for {ph.replace('ESOTERIC_', '').replace('_', ' ').title()}\n")
        if rules_list:
            output_lines.append(f"    // Covers the following rules:\n")
            for rn, txt in sorted(rules_list, key=lambda x: [int(v) for v in re.findall(r'\d+', x[0])] + [x[0]]):
                output_lines.append(f"    // {rn}. {txt}\n")
        else:
            output_lines.append(f"    // (No rules covered currently)\n")
        output_lines.append(f"    {ph},\n\n")

    # Group core granular variants by chapter
    chapters = {
        "1": "Game Concepts",
        "2": "Parts of a Card",
        "3": "Card Types",
        "4": "Zones",
        "5": "Turn Structure",
        "6": "Spells, Abilities, and Effects",
        "7": "Additional Rules",
        "8": "Multiplayer Rules",
        "9": "Casual Variants"
    }
    
    variants_by_chapter = {ch: [] for ch in chapters.keys()}
    for group_dict in ordered_groups:
        if group_dict["is_esoteric"]:
            continue
        
        rnum = group_dict["rules"][0][0]
        ch = rnum[0]
        if ch in variants_by_chapter:
            variants_by_chapter[ch].append(group_dict)
            
    for ch in sorted(variants_by_chapter.keys()):
        ch_name = chapters[ch]
        infos = variants_by_chapter[ch]
        if not infos:
            continue
            
        output_lines.append(f"    // --- {ch}. {ch_name} ---\n\n")
        
        # Sort the groups within each chapter numerically by their first rule number
        infos.sort(key=lambda x: [int(v) for v in re.findall(r'\d+', x["rules"][0][0])] + [x["rules"][0][0]])
        
        for info in infos:
            for rn, txt in sorted(info["rules"], key=lambda x: [int(v) for v in re.findall(r'\d+', x[0])] + [x[0]]):
                output_lines.append(f"    // {rn}. {txt}\n")
                
            v_def = info["variant_name"]
            if info["has_condition"]:
                v_def += "(Condition)"
            output_lines.append(f"    {v_def},\n\n")

    output_lines.append("}\n")

    with open(rules_rs_path, "w", encoding="utf-8") as f:
        f.writelines(output_lines)

    print(f"Success! Generated {len(ordered_groups) - len(esoteric_placeholders_list)} core granular variants and {len(esoteric_placeholders_list)} esoteric placeholder variants.")
    print("Added them to src/compiler/rules.rs successfully.")

if __name__ == "__main__":
    main()

