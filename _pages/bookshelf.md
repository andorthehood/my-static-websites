---
layout: post
title: 'Bookshelf'
date: 2023-07-19 19:00:00 -0100
location: 'Zandvoort, Netherlands'
---

<style>

   .bookshelf {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 10px; /* Adjust the gap between items as needed */
    }

    .bookshelf li {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        aspect-ratio: 3 / 4;
        text-align: center;
        word-break: break-word;
        padding: 10px;
        box-sizing: border-box; 
        position: relative;
    }

    .bookshelf .small {
        font-size: 11px;
        line-height: 11px;
        padding: 5px 0;
    }

    .bookshelf .upp {
        text-transform: uppercase;
    }

    .bookshelf .left {
        text-align: left;
    }
</style>

<p>Welcome to my virtual bookshelf!</p>

<ul class="bookshelf">
    <li style="background: #fff;"><span class="small" style="color: #c60">Nick Bostrom</span> <span
            class="big">SUPERINTELLIGENCE</span> <span class="small">Paths, Dangers, Strategies</span></li>
    <li style="background: #fcc;"> <span class="big" style="color: #f30">The Alignment Problem</span> <span
            class="small">Machine Learning and Human Values</span> <span class="small" style="color: #f30">Brian
            Christian</span></li>
    <li style="background: #36f;"> <span class="big" style="color: #fff">Being You</span> <span class="small"
            style="color: #fff">ANIL SETH</span> <span class="small" style="color: #fff">A New Science of
            Consciousness</span> </li>
    <li style="background: #fff;"> <span class="big" style="color: #000">A THOUSAND BRAINS</span> <span class="small"
            style="color: #f30">A NEW THEORY OF INTELLIGENCE</span> <span class="small" style="color: #000">JEFF
            HAWKINS</span> </li>
    <li style="background: #fff;"> <span class="small" style="color: #000">THE SCIENCE OF THE MIND AND THE MYTH OF THE SELF</span> <span
            style="color: #f30">THE EGO TUNNEL</span> <span class="small" style="color: #000">THOMAS METZINGER</span> </li>
<li style="background: #000;"> <span class="small" style="color: #6699cc">WHEN HUMANS TRANSCEND BIOLOGY</span> <span
            style="color: #fff">THE SINGULARITY IS NEAR</span> <span class="small" style="color: #fc6">RAY KURZWEIL</span> </li>

<li style="background: #ffc;">
<span class="small upp" style="color: #000">Evolution, AI, and the Five Breakthroughs That Made Our Brains</span> 
<span class="upp" style="color: #000">A Brief History of Intelligence</span>
<span class="small upp" style="color: #000">Max Bennett</span>
</li>

<li style="background: #369;">
<span class="upp left" style="color: #fff">Global Catastrophic Risks</span>
<span class="small upp" style="color: #fff">Nick Bostrom and Milan M. Ćirković</span>
</li>

<li style="background: #fff;">
<span class="upp" style="color: #000">Behave</span>
<span class="upp small" style="color: #000">The Biology of Humans at Our Best and Worst</span>
<span class="upp" style="color: #000">Robert M. Sapolsky</span>
</li>

</ul>
