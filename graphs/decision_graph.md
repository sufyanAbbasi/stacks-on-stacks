# State-Space BFS Decision Graph Comparison

This document contains a side-by-side comparison of how continuous effects—specifically spell taxation (such as *Thalia, Guardian of Thraben*) and activated ability suppression (such as *Grand Abolisher*)—dynamically alter and prune the game's algorithmic state-space decision tree.

## Side-by-Side Comparison Metrics

| Scenario | Active Continuous Effects | BFS Nodes | BFS Edges | Node Reduction % | Edge Reduction % |
| --- | --- | --- | --- | --- | --- |
| **Standard (A)** | None | 142 | 218 | Baseline | Baseline |
| **Thalia Tax (B)** | Spell Taxation +{1} (Noncreatures) | 112 | 168 | 21.1% | 22.9% |
| **Grand Abolisher (C)** | Suppress Opponent Activated Abilities | 26 | 31 | 81.7% | 85.8% |

## Analytical Diagnostics

> [!IMPORTANT]
> **Active Tax and Suppress Effects Prove Dynamic Decision Modification**
> Our priority BFS state-space search successfully proves that MtG card abilities do not just act as static text, but dynamically reshape the future decision-tree path. Active static effects act as powerful filters that block, modify, or completely eliminate decision vertices.

### 1. Standard Scenario Analysis
- Player A has 2 lands (Forest, Mountain) and Fireball in hand. They can make individual decisions to tap Forest, tap Mountain, and cast Fireball for X=0 or X=1.
- Player B has 2 Islands and Counterspell. If Player A casts Fireball, Player B can respond by tapping their Islands and casting Counterspell.

### 2. Thalia Spell Taxation Analysis
- Fireball is taxed by `{1}`. Casting Fireball with X=0 now costs `{1}{R}`. Casting for X=1 costs `{2}{R}`, which Player A cannot pay. Thus, the choice of casting for X=1 is filtered out.
- Counterspell is also taxed by `{1}`, raising its cost to `{2}{U}`. Player B has only 2 Islands, so they are completely locked out of responding. This entire counter-strategy branch is pruned.

### 3. Grand Abolisher Suppression Analysis
- Opponent's activated abilities are suppressed. Tapping an Island for mana is an activated ability (mana ability). Player B is forbidden from activating it.
- Consequently, Player B cannot add `{U}` to pay for Counterspell, ensuring Player A can execute their plays completely uninterrupted.

## Scenario A (Standard) Decision Graph Flowchart

# MTG Decision Graph & Priority Flowchart

This document contains the auto-generated **Mermaid.js flowchart** of our priority-state decision graph.

```mermaid
flowchart TD
    %% Node styles and definitions
    classDef apColor fill:#2ecc71,stroke:#27ae60,stroke-width:2px,color:#fff;
    classDef napColor fill:#3498db,stroke:#2980b9,stroke-width:2px,color:#fff;
    classDef systemColor fill:#9b59b6,stroke:#8e44ad,stroke-width:2px,color:#fff;

    Node_39("**Node 39**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_39 napColor;
    Node_110("**Node 110**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_110 apColor;
    Node_141["**Node 141**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0."]
    class Node_141 systemColor;
    Node_27("**Node 27**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_27 apColor;
    Node_88("**Node 88**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_88 apColor;
    Node_124("**Node 124**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_124 napColor;
    Node_30("**Node 30**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_30 apColor;
    Node_47("**Node 47**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_47 apColor;
    Node_93("**Node 93**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_93 apColor;
    Node_34["**Node 34**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_34 systemColor;
    Node_44("**Node 44**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_44 napColor;
    Node_49("**Node 49**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_49 apColor;
    Node_80("**Node 80**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_80 napColor;
    Node_107("**Node 107**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_107 apColor;
    Node_111("**Node 111**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_111 napColor;
    Node_73("**Node 73**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_73 napColor;
    Node_25("**Node 25**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_25 apColor;
    Node_127["**Node 127**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_127 systemColor;
    Node_132("**Node 132**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_132 apColor;
    Node_40("**Node 40**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_40 apColor;
    Node_65("**Node 65**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_65 apColor;
    Node_6("**Node 6**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_6 napColor;
    Node_36("**Node 36**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_36 napColor;
    Node_26("**Node 26**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_26 apColor;
    Node_101("**Node 101**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_101 napColor;
    Node_91("**Node 91**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_91 napColor;
    Node_75("**Node 75**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_75 apColor;
    Node_89["**Node 89**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_89 systemColor;
    Node_69("**Node 69**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_69 napColor;
    Node_58("**Node 58**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_58 apColor;
    Node_77("**Node 77**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_77 apColor;
    Node_96("**Node 96**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_96 apColor;
    Node_20["**Node 20**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_20 systemColor;
    Node_100("**Node 100**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_100 napColor;
    Node_23("**Node 23**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_23 napColor;
    Node_90["**Node 90**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_90 systemColor;
    Node_97("**Node 97**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_97 napColor;
    Node_48["**Node 48**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_48 systemColor;
    Node_105("**Node 105**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_105 napColor;
    Node_15["**Node 15**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_15 systemColor;
    Node_115("**Node 115**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_115 apColor;
    Node_76("**Node 76**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_76 napColor;
    Node_56("**Node 56**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_56 apColor;
    Node_70("**Node 70**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_70 apColor;
    Node_84["**Node 84**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_84 systemColor;
    Node_5["**Node 5**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_5 systemColor;
    Node_128("**Node 128**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_128 napColor;
    Node_35("**Node 35**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_35 napColor;
    Node_54("**Node 54**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_54 apColor;
    Node_125["**Node 125**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_125 systemColor;
    Node_10("**Node 10**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_10 napColor;
    Node_99("**Node 99**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_99 apColor;
    Node_13("**Node 13**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_13 napColor;
    Node_67("**Node 67**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_67 napColor;
    Node_19("**Node 19**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_19 apColor;
    Node_92("**Node 92**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_92 napColor;
    Node_142["**Node 142**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0."]
    class Node_142 systemColor;
    Node_113("**Node 113**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_113 napColor;
    Node_83("**Node 83**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_83 apColor;
    Node_7("**Node 7**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_7 napColor;
    Node_120("**Node 120**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_120 apColor;
    Node_17("**Node 17**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_17 napColor;
    Node_8("**Node 8**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_8 napColor;
    Node_126("**Node 126**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_126 apColor;
    Node_79("**Node 79**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_79 apColor;
    Node_24["**Node 24**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_24 systemColor;
    Node_68("**Node 68**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_68 apColor;
    Node_33("**Node 33**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_33 apColor;
    Node_131["**Node 131**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_131 systemColor;
    Node_46("**Node 46**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_46 napColor;
    Node_87("**Node 87**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_87 napColor;
    Node_2("**Node 2**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_2 napColor;
    Node_53("**Node 53**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_53 napColor;
    Node_86("**Node 86**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_86 apColor;
    Node_9("**Node 9**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_9 apColor;
    Node_109("**Node 109**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_109 apColor;
    Node_18("**Node 18**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_18 napColor;
    Node_57["**Node 57**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_57 systemColor;
    Node_134("**Node 134**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_134 napColor;
    Node_135("**Node 135**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_135 apColor;
    Node_31("**Node 31**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_31 apColor;
    Node_51("**Node 51**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_51 napColor;
    Node_50("**Node 50**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_50 apColor;
    Node_41("**Node 41**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_41 apColor;
    Node_139["**Node 139**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_139 systemColor;
    Node_63("**Node 63**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_63 napColor;
    Node_119("**Node 119**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_119 napColor;
    Node_32("**Node 32**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_32 napColor;
    Node_78("**Node 78**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_78 napColor;
    Node_81["**Node 81**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_81 systemColor;
    Node_82["**Node 82**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_82 systemColor;
    Node_11("**Node 11**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_11 apColor;
    Node_94("**Node 94**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_94 apColor;
    Node_72("**Node 72**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_72 apColor;
    Node_1_Start("**Node 1 Start**<br/>Initial State: Player A has priority.")
    class Node_1_Start apColor;
    Node_98("**Node 98**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_98 napColor;
    Node_59("**Node 59**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_59 napColor;
    Node_129("**Node 129**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_129 apColor;
    Node_29("**Node 29**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_29 apColor;
    Node_60("**Node 60**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_60 apColor;
    Node_42("**Node 42**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_42 napColor;
    Node_130("**Node 130**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_130 apColor;
    Node_38("**Node 38**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_38 apColor;
    Node_71("**Node 71**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_71 napColor;
    Node_140("**Node 140**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_140 napColor;
    Node_61("**Node 61**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_61 apColor;
    Node_112("**Node 112**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_112 apColor;
    Node_123("**Node 123**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_123 apColor;
    Node_3("**Node 3**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_3 apColor;
    Node_28["**Node 28**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_28 systemColor;
    Node_104("**Node 104**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_104 napColor;
    Node_136["**Node 136**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_136 systemColor;
    Node_21("**Node 21**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_21 napColor;
    Node_14("**Node 14**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_14 apColor;
    Node_22("**Node 22**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_22 napColor;
    Node_45("**Node 45**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_45 apColor;
    Node_62("**Node 62**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_62 napColor;
    Node_121("**Node 121**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_121 napColor;
    Node_137("**Node 137**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_137 napColor;
    Node_74("**Node 74**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_74 napColor;
    Node_133["**Node 133**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_133 systemColor;
    Node_85("**Node 85**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_85 napColor;
    Node_102["**Node 102**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_102 systemColor;
    Node_16("**Node 16**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_16 napColor;
    Node_37("**Node 37**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_37 napColor;
    Node_114("**Node 114**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_114 apColor;
    Node_66["**Node 66**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_66 systemColor;
    Node_43("**Node 43**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_43 napColor;
    Node_95("**Node 95**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_95 napColor;
    Node_117("**Node 117**<br/>Stack: [Fireball (cast by Player A)]. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_117 napColor;
    Node_103["**Node 103**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1."]
    class Node_103 systemColor;
    Node_118("**Node 118**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_118 apColor;
    Node_52("**Node 52**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_52 apColor;
    Node_122("**Node 122**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_122 apColor;
    Node_12("**Node 12**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_12 apColor;
    Node_55["**Node 55**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_55 systemColor;
    Node_106("**Node 106**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_106 apColor;
    Node_108("**Node 108**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_108 napColor;
    Node_116("**Node 116**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 1.")
    class Node_116 napColor;
    Node_138("**Node 138**<br/>Stack: [Fireball (cast by Player A) -> Counterspell (cast by Player B)]. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 0, Player B: 0.")
    class Node_138 napColor;
    Node_64["**Node 64**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_64 systemColor;
    Node_4("**Node 4**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_4 apColor;

    %% Transitions / Edges
    Node_1_Start -->|"Player A Passes"| Node_2
    Node_1_Start -->|"Player A Taps Forest"| Node_3
    Node_1_Start -->|"Player A Taps Mountain"| Node_4
    Node_2 -->|"Player B Passes"| Node_5
    Node_2 -->|"Player B Taps Island"| Node_6
    Node_2 -->|"Player B Taps Island"| Node_7
    Node_3 -->|"Player A Passes"| Node_8
    Node_3 -->|"Player A Taps Mountain"| Node_9
    Node_4 -->|"Player A Passes"| Node_10
    Node_4 -->|"Player A Taps Forest"| Node_9
    Node_4 -->|"Player A Casts Fireball"| Node_11
    Node_6 -->|"Player B Passes"| Node_12
    Node_6 -->|"Player B Taps Island"| Node_13
    Node_7 -->|"Player B Passes"| Node_14
    Node_7 -->|"Player B Taps Island"| Node_13
    Node_8 -->|"Player B Passes"| Node_15
    Node_8 -->|"Player B Taps Island"| Node_16
    Node_8 -->|"Player B Taps Island"| Node_17
    Node_9 -->|"Player A Passes"| Node_18
    Node_9 -->|"Player A Casts Fireball"| Node_19
    Node_10 -->|"Player B Passes"| Node_20
    Node_10 -->|"Player B Taps Island"| Node_21
    Node_10 -->|"Player B Taps Island"| Node_22
    Node_11 -->|"Player A Passes"| Node_23
    Node_11 -->|"Player A Taps Forest"| Node_19
    Node_12 -->|"Player A Passes"| Node_24
    Node_12 -->|"Player A Taps Forest"| Node_25
    Node_12 -->|"Player A Taps Mountain"| Node_26
    Node_13 -->|"Player B Passes"| Node_27
    Node_14 -->|"Player A Passes"| Node_28
    Node_14 -->|"Player A Taps Forest"| Node_29
    Node_14 -->|"Player A Taps Mountain"| Node_30
    Node_16 -->|"Player B Passes"| Node_31
    Node_16 -->|"Player B Taps Island"| Node_32
    Node_17 -->|"Player B Passes"| Node_33
    Node_17 -->|"Player B Taps Island"| Node_32
    Node_18 -->|"Player B Passes"| Node_34
    Node_18 -->|"Player B Taps Island"| Node_35
    Node_18 -->|"Player B Taps Island"| Node_36
    Node_19 -->|"Player A Passes"| Node_37
    Node_21 -->|"Player B Passes"| Node_38
    Node_21 -->|"Player B Taps Island"| Node_39
    Node_22 -->|"Player B Passes"| Node_40
    Node_22 -->|"Player B Taps Island"| Node_39
    Node_23 -->|"Player B Passes (Fireball resolves.)"| Node_41
    Node_23 -->|"Player B Taps Island"| Node_42
    Node_23 -->|"Player B Taps Island"| Node_43
    Node_25 -->|"Player A Passes"| Node_44
    Node_25 -->|"Player A Taps Mountain"| Node_45
    Node_26 -->|"Player A Passes"| Node_46
    Node_26 -->|"Player A Taps Forest"| Node_45
    Node_26 -->|"Player A Casts Fireball"| Node_47
    Node_27 -->|"Player A Passes"| Node_48
    Node_27 -->|"Player A Taps Forest"| Node_49
    Node_27 -->|"Player A Taps Mountain"| Node_50
    Node_29 -->|"Player A Passes"| Node_51
    Node_29 -->|"Player A Taps Mountain"| Node_52
    Node_30 -->|"Player A Passes"| Node_53
    Node_30 -->|"Player A Taps Forest"| Node_52
    Node_30 -->|"Player A Casts Fireball"| Node_54
    Node_31 -->|"Player A Passes"| Node_55
    Node_31 -->|"Player A Taps Mountain"| Node_45
    Node_32 -->|"Player B Passes"| Node_56
    Node_33 -->|"Player A Passes"| Node_57
    Node_33 -->|"Player A Taps Mountain"| Node_52
    Node_35 -->|"Player B Passes"| Node_58
    Node_35 -->|"Player B Taps Island"| Node_59
    Node_36 -->|"Player B Passes"| Node_60
    Node_36 -->|"Player B Taps Island"| Node_59
    Node_37 -->|"Player B Passes (Fireball resolves.)"| Node_61
    Node_37 -->|"Player B Taps Island"| Node_62
    Node_37 -->|"Player B Taps Island"| Node_63
    Node_38 -->|"Player A Passes"| Node_64
    Node_38 -->|"Player A Taps Forest"| Node_45
    Node_38 -->|"Player A Casts Fireball"| Node_47
    Node_39 -->|"Player B Passes"| Node_65
    Node_40 -->|"Player A Passes"| Node_66
    Node_40 -->|"Player A Taps Forest"| Node_52
    Node_40 -->|"Player A Casts Fireball"| Node_54
    Node_41 -->|"Player A Passes"| Node_67
    Node_41 -->|"Player A Taps Forest"| Node_61
    Node_42 -->|"Player B Passes"| Node_68
    Node_42 -->|"Player B Taps Island"| Node_69
    Node_43 -->|"Player B Passes"| Node_70
    Node_43 -->|"Player B Taps Island"| Node_69
    Node_44 -->|"Player B Passes"| Node_55
    Node_44 -->|"Player B Taps Island"| Node_32
    Node_45 -->|"Player A Passes"| Node_71
    Node_45 -->|"Player A Casts Fireball"| Node_72
    Node_46 -->|"Player B Passes"| Node_64
    Node_46 -->|"Player B Taps Island"| Node_39
    Node_47 -->|"Player A Passes"| Node_73
    Node_47 -->|"Player A Taps Forest"| Node_72
    Node_49 -->|"Player A Passes"| Node_74
    Node_49 -->|"Player A Taps Mountain"| Node_75
    Node_50 -->|"Player A Passes"| Node_76
    Node_50 -->|"Player A Taps Forest"| Node_75
    Node_50 -->|"Player A Casts Fireball"| Node_77
    Node_51 -->|"Player B Passes"| Node_57
    Node_51 -->|"Player B Taps Island"| Node_32
    Node_52 -->|"Player A Passes"| Node_78
    Node_52 -->|"Player A Casts Fireball"| Node_79
    Node_53 -->|"Player B Passes"| Node_66
    Node_53 -->|"Player B Taps Island"| Node_39
    Node_54 -->|"Player A Passes"| Node_80
    Node_54 -->|"Player A Taps Forest"| Node_79
    Node_56 -->|"Player A Passes"| Node_81
    Node_56 -->|"Player A Taps Mountain"| Node_75
    Node_58 -->|"Player A Passes"| Node_82
    Node_58 -->|"Player A Casts Fireball"| Node_72
    Node_59 -->|"Player B Passes"| Node_83
    Node_60 -->|"Player A Passes"| Node_84
    Node_60 -->|"Player A Casts Fireball"| Node_79
    Node_61 -->|"Player A Passes"| Node_85
    Node_62 -->|"Player B Passes"| Node_86
    Node_62 -->|"Player B Taps Island"| Node_87
    Node_63 -->|"Player B Passes"| Node_88
    Node_63 -->|"Player B Taps Island"| Node_87
    Node_65 -->|"Player A Passes"| Node_89
    Node_65 -->|"Player A Taps Forest"| Node_75
    Node_65 -->|"Player A Casts Fireball"| Node_77
    Node_67 -->|"Player B Passes"| Node_90
    Node_67 -->|"Player B Taps Island"| Node_91
    Node_67 -->|"Player B Taps Island"| Node_92
    Node_68 -->|"Player A Passes (Fireball resolves.)"| Node_93
    Node_68 -->|"Player A Taps Forest"| Node_72
    Node_69 -->|"Player B Passes"| Node_94
    Node_69 -->|"Player B Casts Counterspell"| Node_95
    Node_70 -->|"Player A Passes (Fireball resolves.)"| Node_96
    Node_70 -->|"Player A Taps Forest"| Node_79
    Node_71 -->|"Player B Passes"| Node_82
    Node_71 -->|"Player B Taps Island"| Node_59
    Node_72 -->|"Player A Passes"| Node_97
    Node_73 -->|"Player B Passes (Fireball resolves.)"| Node_93
    Node_73 -->|"Player B Taps Island"| Node_69
    Node_74 -->|"Player B Passes"| Node_81
    Node_75 -->|"Player A Passes"| Node_98
    Node_75 -->|"Player A Casts Fireball"| Node_99
    Node_76 -->|"Player B Passes"| Node_89
    Node_77 -->|"Player A Passes"| Node_100
    Node_77 -->|"Player A Taps Forest"| Node_99
    Node_78 -->|"Player B Passes"| Node_84
    Node_78 -->|"Player B Taps Island"| Node_59
    Node_79 -->|"Player A Passes"| Node_101
    Node_80 -->|"Player B Passes (Fireball resolves.)"| Node_96
    Node_80 -->|"Player B Taps Island"| Node_69
    Node_83 -->|"Player A Passes"| Node_102
    Node_83 -->|"Player A Casts Fireball"| Node_99
    Node_85 -->|"Player B Passes"| Node_103
    Node_85 -->|"Player B Taps Island"| Node_104
    Node_85 -->|"Player B Taps Island"| Node_105
    Node_86 -->|"Player A Passes (Fireball resolves.)"| Node_106
    Node_87 -->|"Player B Passes"| Node_107
    Node_87 -->|"Player B Casts Counterspell"| Node_108
    Node_88 -->|"Player A Passes (Fireball resolves.)"| Node_109
    Node_91 -->|"Player B Passes"| Node_110
    Node_91 -->|"Player B Taps Island"| Node_111
    Node_92 -->|"Player B Passes"| Node_112
    Node_92 -->|"Player B Taps Island"| Node_111
    Node_93 -->|"Player A Passes"| Node_113
    Node_93 -->|"Player A Taps Forest"| Node_106
    Node_94 -->|"Player A Passes (Fireball resolves.)"| Node_114
    Node_94 -->|"Player A Taps Forest"| Node_99
    Node_95 -->|"Player B Passes"| Node_115
    Node_96 -->|"Player A Passes"| Node_116
    Node_96 -->|"Player A Taps Forest"| Node_109
    Node_97 -->|"Player B Passes (Fireball resolves.)"| Node_106
    Node_97 -->|"Player B Taps Island"| Node_87
    Node_98 -->|"Player B Passes"| Node_102
    Node_99 -->|"Player A Passes"| Node_117
    Node_100 -->|"Player B Passes (Fireball resolves.)"| Node_114
    Node_100 -->|"Player B Casts Counterspell"| Node_95
    Node_101 -->|"Player B Passes (Fireball resolves.)"| Node_109
    Node_101 -->|"Player B Taps Island"| Node_87
    Node_104 -->|"Player B Passes"| Node_118
    Node_104 -->|"Player B Taps Island"| Node_119
    Node_105 -->|"Player B Passes"| Node_120
    Node_105 -->|"Player B Taps Island"| Node_119
    Node_106 -->|"Player A Passes"| Node_121
    Node_107 -->|"Player A Passes (Fireball resolves.)"| Node_122
    Node_108 -->|"Player B Passes"| Node_123
    Node_109 -->|"Player A Passes"| Node_124
    Node_110 -->|"Player A Passes"| Node_125
    Node_110 -->|"Player A Taps Forest"| Node_106
    Node_111 -->|"Player B Passes"| Node_126
    Node_112 -->|"Player A Passes"| Node_127
    Node_112 -->|"Player A Taps Forest"| Node_109
    Node_113 -->|"Player B Passes"| Node_125
    Node_113 -->|"Player B Taps Island"| Node_111
    Node_114 -->|"Player A Passes"| Node_128
    Node_114 -->|"Player A Taps Forest"| Node_122
    Node_115 -->|"Player A Passes (Counterspell resolves. Fireball is countered and put into owner's graveyard.)"| Node_129
    Node_115 -->|"Player A Taps Forest"| Node_130
    Node_116 -->|"Player B Passes"| Node_127
    Node_116 -->|"Player B Taps Island"| Node_111
    Node_117 -->|"Player B Passes (Fireball resolves.)"| Node_122
    Node_117 -->|"Player B Casts Counterspell"| Node_108
    Node_118 -->|"Player A Passes"| Node_131
    Node_119 -->|"Player B Passes"| Node_132
    Node_120 -->|"Player A Passes"| Node_133
    Node_121 -->|"Player B Passes"| Node_131
    Node_121 -->|"Player B Taps Island"| Node_119
    Node_122 -->|"Player A Passes"| Node_134
    Node_123 -->|"Player A Passes (Counterspell resolves. Fireball is countered and put into owner's graveyard.)"| Node_135
    Node_124 -->|"Player B Passes"| Node_133
    Node_124 -->|"Player B Taps Island"| Node_119
    Node_126 -->|"Player A Passes"| Node_136
    Node_126 -->|"Player A Taps Forest"| Node_122
    Node_128 -->|"Player B Passes"| Node_136
    Node_129 -->|"Player A Passes"| Node_137
    Node_129 -->|"Player A Taps Forest"| Node_135
    Node_130 -->|"Player A Passes"| Node_138
    Node_132 -->|"Player A Passes"| Node_139
    Node_134 -->|"Player B Passes"| Node_139
    Node_135 -->|"Player A Passes"| Node_140
    Node_137 -->|"Player B Passes"| Node_141
    Node_138 -->|"Player B Passes (Counterspell resolves. Fireball is countered and put into owner's graveyard.)"| Node_135
    Node_140 -->|"Player B Passes"| Node_142
```
