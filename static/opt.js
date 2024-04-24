var fadeOutTime = 4000;
$(document).ready(function () {
    $("#key").keydown(function (event) {
        if (event.keyCode == 13) {
            search();
        }
    });
});
function search() {
    var key = $("#key").val();
    if (key == "") {
        $("#key").focus();
        return;
    }
    $.ajax({
        type: 'POST',
        url: "scan",
        headers: {'Content-Type': 'application/x-www-form-urlencoded'},
        contentType: 'application/x-www-form-urlencoded; charset=utf-8',
        dataType: 'json',
        data: {"key": key},
        success: function (resp) {
            var content = "";
            if (resp.code == 200) {
                // code = 200 是模糊查找，显示找到的所有相关的key
                for(var i = 0; i < resp.data.length; i++) {
                    content += resp.data[i];
                    content += "\n";
                }
            } else if (resp.code == 201) {
                // code = 201 是精确查找，显示key的内容
                content = resp.data;
            } else {
                content = "操作失败"
                $("#tips").html("操作失败");
                $("#tips").show();
                $("#tips").fadeOut(fadeOutTime, function () {
                    $("#tips").hide();
                });
            }
            $("#val-show").val(content);
        }
    })
}
function deleteCache() {
    var key = $("#key").val();
    if (key == "") {
        $("#key").focus();
        return;
    }
    if(confirm("确定要删除该key的缓存吗?")) {
        $.ajax({
            type: 'POST',
            url: "delete",
            headers: {'Content-Type': 'application/x-www-form-urlencoded'},
            contentType: 'application/x-www-form-urlencoded; charset=utf-8',
            dataType: 'json',
            data: {"key": key},
            success: function (resp) {
                if (resp.code == 200) {
                    $("#val-show").val("操作成功");
                    $("#tips").html("操作成功");
                    $("#tips").show();
                    $("#tips").fadeOut(fadeOutTime, function () {
                        $("#tips").hide();
                    });
                } else {
                    $("#val-show").val("操作失败");
                    $("#tips").html("操作失败");
                    $("#tips").show();
                    $("#tips").fadeOut(fadeOutTime, function () {
                        $("#tips").hide();
                    });
                }
            }
        })
    }
}
function addCache() {
    var key = $("#key").val();
    var val = $("#val-show").val();
    if (key == "" || val == "") {
        $("#tips").html("key或value不能为空");
        $("#tips").show();
        $("#tips").fadeOut(fadeOutTime, function () {
            $("#tips").hide();
        });
        return;
    }
    $.ajax({
        type: 'POST',
        url: "set",
        headers: {'Content-Type': 'application/x-www-form-urlencoded'},
        contentType: 'application/x-www-form-urlencoded; charset=utf-8',
        dataType: 'json',
        data: {"key": key, "val": val},
        success: function (resp) {
            if (resp.code == 200) {
                $("#tips").html("操作成功");
                $("#tips").show();
                $("#tips").fadeOut(fadeOutTime, function () {
                    $("#tips").hide();
                });
            } else {
                $("#tips").html("操作失败");
                $("#tips").show();
                $("#tips").fadeOut(fadeOutTime, function () {
                    $("#tips").hide();
                });
            }
        }
    })
}