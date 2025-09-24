---
layout: post
title: 'Fun facts about the DVD logo screensaver'
date: 2023-10-01T23:53+0100
location: 'Zandvoort, Netherlands'
---

<p>I was recently rewatching The Office (S4E5), and the scene where everyone cheers for the DVD logo inspired me to code my own bouncing logo. I wanted to program it in a way so that it always hits a corner within a reasonable time, and along the way, I learned a few facts:</p>

<p>Fact 1: If the DVD logo ever hits a corner, it will exactly retrace its previous path in reverse. In practice, that means it travels back along the same diagonal path.</p>

<p>Fact 2: If it hits one corner, it is guaranteed to hit a second one. Because each corner hit reverses the path of the logo, it gets locked between these two, repeating the cycle forever. It is mathematically impossible for it to ever reach a third or fourth.</p>

<p>Fact 3: The previous one was actually two facts. Also, black bears can run at speeds up to 35 miles per hour.</p>