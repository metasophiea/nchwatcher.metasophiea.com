const loadPadding = 100;

let diffList = [];
let loadedDiffLists = [];

//utils
    function makeEventCard(
        url, //string
        title, //string | undefined
        imageUrl, //string | undefined
        date, //[year, date]number | undefined
        time, //[hour, minute, second, nanosecond]number | undefined
    ){
        const section = document.createElement('section');
        section.setAttribute('class', 'event-list-card');

        const a = document.createElement('a');
        a.setAttribute('class','event-list-card-title');
        a.href = 'https://nch.ie/'+url;
            var h1 = document.createElement('h1');
            h1.innerHTML = title || "-title missing-";
            a.append(h1);
        section.append(a);

        if(date) {
            const datetime = time ? new Date(date[0], 0, date[1], time[0], time[1]) : new Date(date[0], 0, date[1]);
            const isInThePast = datetime - new Date() < 0;

            section.setAttribute(
                'class',
                section.getAttribute('class') + (isInThePast ? ' event-list-card-inThePast' : '')
            );

            const p = document.createElement('p');
            p.setAttribute('class', 'event-list-card-time');
            p.innerHTML = datetime.toLocaleDateString() + (time ? ' - ' + datetime.toLocaleTimeString('en-US', {timeStyle:'short'}) : '');
            section.append(p);
        }

        if(imageUrl) {
            const img = document.createElement('div');
            img.setAttribute('hidden_src',imageUrl);
            img.setAttribute('class','event-list-card-image');
            section.append(img);
        }

        return section;
    }
    function generateEventList(diffName, events) {
        const section = document.createElement('section');
        section.setAttribute('class', 'event-list');

        const infobar = document.createElement('section');
        infobar.setAttribute('class', 'event-list-info');
            const p = document.createElement('p');
            p.setAttribute('class','event-list-info-text');
            const info = diffName.split('>');
            p.innerHTML = events.length == 0 ? 
                `No changes in event listings from <b>${info[0]}</b> to <b>${info[1]}</b>` :
                `Displaying changes in event listings from <b>${info[0]}</b> to <b>${info[1]}</b>`;
            infobar.append(p);
        section.append(infobar);

        events.forEach((event) => {
            let eventCard = makeEventCard(
                event.url,
                event.title,
                event.imageUrl,
                event.date,
                event.time
            );

            section.append(eventCard);
        });

        return section;
    }

//live update
    window.onscroll = function() {
        //event list request
            const eventList = document.getElementsByClassName('event-list');
            const lastEventList = eventList[eventList.length-1];

            let bottomOfLastEventList = lastEventList.getBoundingClientRect().y + lastEventList.getBoundingClientRect().height;
            if( bottomOfLastEventList < window.innerHeight + loadPadding ) {
                const diffName = diffList.pop();
                if(diffName) {
                    fetch(`diff/${diffName}`)
                        .then((response) => response.json())
                        .then((events) => {
                            document.body.append(generateEventList(diffName, events));
                            loadedDiffLists.push(diffName);
                            window.onscroll();
                        });
                }
            }

        //image request
            //check all cards. If one is in frame - and it not already loaded - add the url to the img element
            let cards = document.getElementsByClassName('event-list-card');
            for(let a = 0; a < cards.length; a++){
                let topOfCard = cards[a].getBoundingClientRect().y;
                let bottomOfCard = cards[a].getBoundingClientRect().y + cards[a].getBoundingClientRect().height;

                if(
                    (
                        (
                            0 < topOfCard && topOfCard < window.innerHeight
                        ) || (
                            0 < bottomOfCard && bottomOfCard < window.innerHeight
                        )
                    ) && 
                    cards[a].getElementsByClassName('event-list-card-image')[0].getAttribute("hidden_src") != null
                ){
                    let src = cards[a].getElementsByClassName('event-list-card-image')[0].getAttribute("hidden_src");
                    cards[a].getElementsByClassName('event-list-card-image')[0].style['background-image'] = 'url('+src+')';
                    cards[a].getElementsByClassName('event-list-card-image')[0].removeAttribute("hidden_src")
                }
            }
    };

//initial fetch
    fetch('diff/manifest.json')
        .then((response) => response.json())
        .then((manifest) => {
            diffList = manifest;
            const diffName = diffList.pop();
            fetch(`diff/${diffName}`)
                .then((response) => response.json())
                .then((events) => {
                    document.body.append(generateEventList(diffName, events));
                    loadedDiffLists.push(diffName);
                    window.onscroll();
                });
        });