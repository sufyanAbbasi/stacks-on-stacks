# MTG Decision Graph & Priority Flowchart

This document contains the auto-generated **Mermaid.js flowchart** of our priority-state decision graph.

```mermaid
flowchart TD
    %% Node styles and definitions
    classDef apColor fill:#2ecc71,stroke:#27ae60,stroke-width:2px,color:#fff;
    classDef napColor fill:#3498db,stroke:#2980b9,stroke-width:2px,color:#fff;
    classDef systemColor fill:#9b59b6,stroke:#8e44ad,stroke-width:2px,color:#fff;

    Node_15("**Node 15**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_15 napColor;
    Node_60["**Node 60**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_60 systemColor;
    Node_66["**Node 66**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_66 systemColor;
    Node_22("**Node 22**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_22 apColor;
    Node_65("**Node 65**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_65 napColor;
    Node_24("**Node 24**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_24 apColor;
    Node_29("**Node 29**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_29 napColor;
    Node_9("**Node 9**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_9 apColor;
    Node_35("**Node 35**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_35 napColor;
    Node_2("**Node 2**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_2 napColor;
    Node_39("**Node 39**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_39 napColor;
    Node_45("**Node 45**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_45 napColor;
    Node_50("**Node 50**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_50 napColor;
    Node_4("**Node 4**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_4 apColor;
    Node_6("**Node 6**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_6 napColor;
    Node_28("**Node 28**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_28 apColor;
    Node_5["**Node 5**<br/>Stack is empty. Mana pools: Player A: empty; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_5 systemColor;
    Node_40["**Node 40**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_40 systemColor;
    Node_41("**Node 41**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_41 apColor;
    Node_57("**Node 57**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_57 apColor;
    Node_19("**Node 19**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_19 napColor;
    Node_46["**Node 46**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_46 systemColor;
    Node_10("**Node 10**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_10 napColor;
    Node_12("**Node 12**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_12 napColor;
    Node_61["**Node 61**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_61 systemColor;
    Node_18["**Node 18**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_18 systemColor;
    Node_37("**Node 37**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_37 napColor;
    Node_55("**Node 55**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_55 napColor;
    Node_44("**Node 44**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_44 apColor;
    Node_20("**Node 20**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_20 napColor;
    Node_1_Start("**Node 1 Start**<br/>Initial State: Player A has priority.")
    class Node_1_Start apColor;
    Node_23("**Node 23**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_23 apColor;
    Node_33("**Node 33**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_33 napColor;
    Node_27("**Node 27**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_27 apColor;
    Node_58("**Node 58**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_58 napColor;
    Node_13("**Node 13**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_13 apColor;
    Node_26("**Node 26**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_26 apColor;
    Node_8("**Node 8**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_8 napColor;
    Node_14["**Node 14**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_14 systemColor;
    Node_56("**Node 56**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_56 napColor;
    Node_62("**Node 62**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_62 apColor;
    Node_54["**Node 54**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_54 systemColor;
    Node_63["**Node 63**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_63 systemColor;
    Node_38("**Node 38**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_38 apColor;
    Node_21["**Node 21**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_21 systemColor;
    Node_51("**Node 51**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_51 apColor;
    Node_31["**Node 31**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_31 systemColor;
    Node_52["**Node 52**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_52 systemColor;
    Node_42("**Node 42**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_42 apColor;
    Node_17("**Node 17**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_17 napColor;
    Node_25["**Node 25**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_25 systemColor;
    Node_32("**Node 32**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_32 napColor;
    Node_16("**Node 16**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_16 napColor;
    Node_47("**Node 47**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_47 apColor;
    Node_49("**Node 49**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_49 apColor;
    Node_48["**Node 48**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_48 systemColor;
    Node_64["**Node 64**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1."]
    class Node_64 systemColor;
    Node_30("**Node 30**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_30 apColor;
    Node_59("**Node 59**<br/>Stack is empty. Mana pools: Player A: {R: 1, G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_59 napColor;
    Node_7("**Node 7**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_7 napColor;
    Node_43("**Node 43**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_43 napColor;
    Node_36("**Node 36**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_36 apColor;
    Node_53("**Node 53**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 2}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_53 apColor;
    Node_34("**Node 34**<br/>Stack is empty. Mana pools: Player A: {R: 1}; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_34 apColor;
    Node_11("**Node 11**<br/>Stack is empty. Mana pools: Player A: empty; Player B: {U: 1}. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_11 apColor;
    Node_3("**Node 3**<br/>Stack is empty. Mana pools: Player A: {G: 1}; Player B: empty. Battlefield: Runeclaw Bear (ID: 30, damage: 0). Hands: Player A: 1, Player B: 1.")
    class Node_3 apColor;

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
    Node_6 -->|"Player B Passes"| Node_11
    Node_6 -->|"Player B Taps Island"| Node_12
    Node_7 -->|"Player B Passes"| Node_13
    Node_7 -->|"Player B Taps Island"| Node_12
    Node_8 -->|"Player B Passes"| Node_14
    Node_8 -->|"Player B Taps Island"| Node_15
    Node_8 -->|"Player B Taps Island"| Node_16
    Node_9 -->|"Player A Passes"| Node_17
    Node_10 -->|"Player B Passes"| Node_18
    Node_10 -->|"Player B Taps Island"| Node_19
    Node_10 -->|"Player B Taps Island"| Node_20
    Node_11 -->|"Player A Passes"| Node_21
    Node_11 -->|"Player A Taps Forest"| Node_22
    Node_11 -->|"Player A Taps Mountain"| Node_23
    Node_12 -->|"Player B Passes"| Node_24
    Node_13 -->|"Player A Passes"| Node_25
    Node_13 -->|"Player A Taps Forest"| Node_26
    Node_13 -->|"Player A Taps Mountain"| Node_27
    Node_15 -->|"Player B Passes"| Node_28
    Node_15 -->|"Player B Taps Island"| Node_29
    Node_16 -->|"Player B Passes"| Node_30
    Node_16 -->|"Player B Taps Island"| Node_29
    Node_17 -->|"Player B Passes"| Node_31
    Node_17 -->|"Player B Taps Island"| Node_32
    Node_17 -->|"Player B Taps Island"| Node_33
    Node_19 -->|"Player B Passes"| Node_34
    Node_19 -->|"Player B Taps Island"| Node_35
    Node_20 -->|"Player B Passes"| Node_36
    Node_20 -->|"Player B Taps Island"| Node_35
    Node_22 -->|"Player A Passes"| Node_37
    Node_22 -->|"Player A Taps Mountain"| Node_38
    Node_23 -->|"Player A Passes"| Node_39
    Node_23 -->|"Player A Taps Forest"| Node_38
    Node_24 -->|"Player A Passes"| Node_40
    Node_24 -->|"Player A Taps Forest"| Node_41
    Node_24 -->|"Player A Taps Mountain"| Node_42
    Node_26 -->|"Player A Passes"| Node_43
    Node_26 -->|"Player A Taps Mountain"| Node_44
    Node_27 -->|"Player A Passes"| Node_45
    Node_27 -->|"Player A Taps Forest"| Node_44
    Node_28 -->|"Player A Passes"| Node_46
    Node_28 -->|"Player A Taps Mountain"| Node_38
    Node_29 -->|"Player B Passes"| Node_47
    Node_30 -->|"Player A Passes"| Node_48
    Node_30 -->|"Player A Taps Mountain"| Node_44
    Node_32 -->|"Player B Passes"| Node_49
    Node_32 -->|"Player B Taps Island"| Node_50
    Node_33 -->|"Player B Passes"| Node_51
    Node_33 -->|"Player B Taps Island"| Node_50
    Node_34 -->|"Player A Passes"| Node_52
    Node_34 -->|"Player A Taps Forest"| Node_38
    Node_35 -->|"Player B Passes"| Node_53
    Node_36 -->|"Player A Passes"| Node_54
    Node_36 -->|"Player A Taps Forest"| Node_44
    Node_37 -->|"Player B Passes"| Node_46
    Node_37 -->|"Player B Taps Island"| Node_29
    Node_38 -->|"Player A Passes"| Node_55
    Node_39 -->|"Player B Passes"| Node_52
    Node_39 -->|"Player B Taps Island"| Node_35
    Node_41 -->|"Player A Passes"| Node_56
    Node_41 -->|"Player A Taps Mountain"| Node_57
    Node_42 -->|"Player A Passes"| Node_58
    Node_42 -->|"Player A Taps Forest"| Node_57
    Node_43 -->|"Player B Passes"| Node_48
    Node_43 -->|"Player B Taps Island"| Node_29
    Node_44 -->|"Player A Passes"| Node_59
    Node_45 -->|"Player B Passes"| Node_54
    Node_45 -->|"Player B Taps Island"| Node_35
    Node_47 -->|"Player A Passes"| Node_60
    Node_47 -->|"Player A Taps Mountain"| Node_57
    Node_49 -->|"Player A Passes"| Node_61
    Node_50 -->|"Player B Passes"| Node_62
    Node_51 -->|"Player A Passes"| Node_63
    Node_53 -->|"Player A Passes"| Node_64
    Node_53 -->|"Player A Taps Forest"| Node_57
    Node_55 -->|"Player B Passes"| Node_61
    Node_55 -->|"Player B Taps Island"| Node_50
    Node_56 -->|"Player B Passes"| Node_60
    Node_57 -->|"Player A Passes"| Node_65
    Node_58 -->|"Player B Passes"| Node_64
    Node_59 -->|"Player B Passes"| Node_63
    Node_59 -->|"Player B Taps Island"| Node_50
    Node_62 -->|"Player A Passes"| Node_66
    Node_65 -->|"Player B Passes"| Node_66
```
