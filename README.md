# PacketTerror - Proiect MDS
Experience the true horrors of networking!

[Cerintele Proiectului](https://app.box.com/notes/46831554845?s=6ly7x02gnt1i3yyjb5hec4no4narasnu)
## Echipa 
- [Vlad Popescu](https://github.com/PVDoriginal)
- [Matei Pescariu](https://github.com/PescaMA)
- [Mihai Duzi](https://github.com/mihaid-asm)

## Descriere
Un joc [Tower Defense](https://en.wikipedia.org/wiki/Tower_defense) cu o tema bazata pe [Cisco Packet Tracer](https://www.netacad.com/cisco-packet-tracer) in care jucatorii protejeaza marele intranet sovietic de atacuri inamice.

Proiectul este realizat in [Bevy](https://bevyengine.org/), un game engine scris in Rust bazat pe design patternul [ECS](https://www.umlboard.com/design-patterns/entity-component-system.html) (Entity Component System) ce prioritizeaza viteza de compilare si paralelizarea codului intr-un mediu non-OOP. 

## UML 
Macro Gameplay Loop: 

```mermaid
flowchart
  A[Main Menu] -- Leave --> B[Exit]
  A -- Play --> C(Level Selection)
  C -- Select Level --> D(In Level)
  D -- Wave Starts --> E(New Wave)
  E -- Build & Upgrade --> E 
  E -- Survive Enemy Packets --> D
  E -- Die --> A 
  E -- Finish Last Wave --> F(Unlock New Level)
  F --> A 
```

Micro Gameplay Loop: 
```mermaid
flowchart
    B[Enemy PC] -- Sends Packet --> C(Packet Destroyed)
    B[Enemy PC] -- Sends Packet --> D[Packet Reached Player]
    D --> F(Health Decreased)
    F -- If Zero --> X[Out] 
    C[Packet Destroyed] -- Player gets Currency --> A
    C -- Last Packet --> X[Out]
    A[Game State] -- Buy Upgrade --> A
    A -- Buy and Place new Item --> A
    
```



## Demo 
[![Watch the video](https://img.youtube.com/vi/xORSITrdFNk/0.jpg)](https://www.youtube.com/watch?v=xORSITrdFNk)

## User Stories
Puteti vedea [aici](https://github.com/users/PVDoriginal/projects/6) backlogul proiectului, format din user stories. 

## Teste Automate 
Avem un fisier de (testing)[https://github.com/PVDoriginal/PacketTerror/blob/main/src/testing.rs] unde putem rula teste automate.

