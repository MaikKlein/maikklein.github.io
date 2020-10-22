$(function() {
    $(".playpen").each(function(block){
        var pre_block = $(this);
        // Add play button
        pre_block.find(".clear-button").click(function(e){
            pre_block.find(".result").text("");
        });
        pre_block.find(".play-button").click(function(e){
            $.ajax({ url: pre_block.data('url'), success: function(data) { run_rust_code(pre_block, data)} });
        });
    });
});

function run_rust_code(block, text) {
    var params = {
        channel: "stable",
        mode: "debug",
        crateType: "bin",
        tests: false,
        code: text,
    }
    var result_block = block.find(".result");
    result_block.text("Running...");

    if (text.indexOf("#![feature") !== -1) {
        params.channel = "nightly";
    }

    //result_block.text("Running...");

    $.ajax({
        url: "https://play.rust-lang.org/execute",
        method: "POST",
        crossDomain: true,
        dataType: "json",
        contentType: "application/json",
        data: JSON.stringify(params),
        timeout: 15000,
        success: function(response) {
            result_block.text(response.success ? response.stdout : response.stderr);
        },
        error: function(qXHR, textStatus, errorThrown) {
            result_block.text("Playground communication " + textStatus);
        },
    });
}
