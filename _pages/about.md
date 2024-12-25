---
layout: post
title: 'About me'
date: 2023-07-19 19:00:00 -0100
location: 'Zandvoort, Netherlands'
---

{% include candystore.liquid file:"20230421_55_ue16_ma_xp2_zm21_tla140" title:"Andor Polgar" %}

<p data-nosnippet id="useragent">It's nice to meet you, #userAgent</p>

<p>My name is Andor Polgar, I was born into a family of first-generation intellectuals right around the time when the Soviet onion was falling apart, and I grew up in the suburbs of a small city in post-communist Hungary. It was a brand new world, not just for me as a baby, but also for my parents and everyone else. It was exciting, but not everything was for the better. A lot of people had, and I think still have a Stockholm syndrome-esque attachment to the old authoritarian ways and corruption, clinging to a past that should have been left behind. That's partly why, after finishing school, I decided that I'm going to take the first oppurtunitiy to start a new life somewhere else.</p>

<p>As a kid, I was hyperfixated on computer programming and technology. I was lucky enough that my parents could afford a computer, however, I didn't have a mentor during those formative years. Eventually, thanks to my fixation, I managed to figure things out on my own, although it took a bit longer. I've also discovered that my brain is wired in a way that makes solving problems feel very rewarding, which has made software engineering the perfect job for me. Nowadays, I work as a freelance software consultant, mostly from my home office in the Netherlands.</p>

<p>I really enjoy taking photos of people and documenting our lives. It's a way for me to connect and create with like-minded artists. I do this purely for my own enjoyment; I'm not after fame, a lasting legacy, or anything like that. While I like the occasional recognition and attention, I'm not concerned about the afterlife of my work. Even if someone ended up using my photos as their own, it wouldn't upset me at all. What matters most to me is the process of creating these images and looking back at them later when I've already forgotten about their existence. They're like little time capsules for me, bringing back memories.</p>

<p>I'm fascinated by the idea of humanity creating an artificial superintelligence. I think it's a very exciting time to be alive! Whenever I'm asked which year I would choose to be reborn in or time travel back to if given the choice, I always choose the present. There's no other era I'd rather experience than the one we're living in right now.</p>

<script>
    const userAgent = navigator.userAgent;
    const textElement = document.getElementById('useragent');
    textElement.innerHTML = textElement.innerHTML.replace('#userAgent', userAgent);
</script>