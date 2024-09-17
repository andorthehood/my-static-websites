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
        height: 200px;
        text-align: center;
        word-break: break-word;
        padding: 10px;
        box-sizing: border-box;  
    }

    .bookshelf .small {
        font-size: 11px;
        line-height: 22px;
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
</ul>
