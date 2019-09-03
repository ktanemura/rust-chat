var _last_update = new Date().toISOString();

function makeMessage(msg) {
    console.log("WAS PASSED", msg)
    let h =`
    <div class="message">
        <strong class="username">< ${msg.time} > ${msg.user}:</strong><p class="message-content">${msg.content}</p>
    </div>`;
    $('#messages').append(h);
}

function messagePoll() {
    let room_id = $('#sendMessageForm').data("room-id");
    let t = new Date().toISOString();
    $.ajax({
        url: `/messages/${room_id}?_d=${_last_update.replace("T", " ").replace("Z", "")}`,
        type: "GET",
        success: function(msgs) {
            _last_update = t;
            console.log("I GOT", msgs);
            for (let i = 0; i < msgs.length; i++) {
                makeMessage(msgs[i]);
            }
        }
    });
}

$(document).ready(function() {
    $('#sendMessageForm').submit(function(e) {
        e.preventDefault();
        $('#warningMsg').hide();
        if ($('#message').val().trim().length > 0) {
            let d = {
                content: $('#message').val().trim(),
                room_id: $('#sendMessageForm').data("room-id")
            }

            $.ajax({
                url: '/messages',
                type: "POST",
                json: true,
                data: d,
                success: function(data) {
                    makeMessage(data);
                    $('#message').val('');
                }
            });
        } else {
            $('#warningMsg').text('Message cannot be empty');
            $('#warningMsg').show();
        }
    });

    setInterval(messagePoll, 3000);
});