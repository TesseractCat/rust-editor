//TODO: Split this into multiple file using "browserify" or "webpack"
window.addEventListener('load', function() {
    external.invoke(JSON.stringify({"type":"init"}));
});
document.addEventListener('keydown', keyDown);

var modeArr = ["normal"];
var actionString = [""];

var actions = {
    "normal": {
        "h":{action:"moveLeft"},
        "j":{action:"moveDown"},
        "k":{action:"moveUp"},
        "l":{action:"moveRight"},
        "<Enter>":{action:"moveDown"},
        "<Backspace>":{action:"moveLeft"},
        
        "<C-k>":{action:"newCursorUp"},
        "<C-j>":{action:"newCursorDown"},
        
        "gg":{action:"moveBeginning"},
        "G":{action:"moveEnd"},
        "w":{action:"moveWord"},
        "$":{action:"moveBack"},
        "0":{action:"moveFront"},
        
        "f":{action:"moveFind",params:["key"]},
        "F":{action:"moveFindReverse",params:["key"]},
        "t":{action:"moveTill",params:["key"]},
        "T":{action:"moveTillReverse",params:["key"]},
        
        "c":{action:["delete","modeInsert"],params:["motion"]},
        "C":"c$",
        "cc":"0c$",
        "r":{action:"replaceChar",params:["key"]},
        "~":{action:["toggleCharCase","moveRight"]},
        "x":{action:"deleteChar"},
        "d":{action:"delete",params:["motion"]},
        "dd":{action:"deleteLine",params:[""]},
        
        "gt":{action:"nextTab"},
        "gT":{action:"prevTab"},
        "o":{action:["openDown","moveDown","modeInsert"]},
        "O":{action:["openUp","modeInsert"]},
        
        "J":{action:"joinLine"},
        
        "I":{action:["moveFront","modeInsert"],params:["local"]},
        "i":{action:"modeInsert",params:["local"]},
        "s":{action:["deleteChar","modeInsert"],params:["local"]},
        "a":{action:["modeInsert","moveRight"],params:["local"]},
        "A":{action:["moveBack","modeInsert","moveRight"],params:["local"]},
        "v":{action:["toggleSelection","modeSelection"]},
        ":":{action:"modeConsole",params:["local"]},
        
        "<C-u>":{action:"viewportUp"},
        "<C-d>":{action:"viewportDown"},
        
        "<A-F4>":"<Escape>:q<Enter>",
        "<C-s>":":w<Enter>",
        "<C-o>":{action:"openFile"},
        
        "<Escape>":{action:"clearActions"},
    },
    "selection": {
        "h":{action:"moveLeft"},
        "j":{action:"moveDown"},
        "k":{action:"moveUp"},
        "l":{action:"moveRight"},
        "o":{action:"swapSelection"},
        
        "gg":{action:"moveBeginning"},
        "G":{action:"moveEnd"},
        "w":{action:"moveWord"},
        "$":{action:"moveBack"},
        "0":{action:"moveFront"},
        
        "~":{action:["toggleCharCase","exitModeSelection"]},
        "r":{action:["replaceChar","exitModeSelection"],params:["key"]},
        "d":{action:["delete","exitModeSelection"]},
        "x":{action:["delete","exitModeSelection"]},
        "c":"di",
        
        "f":{action:"moveFind",params:["key"]},
        "F":{action:"moveFindReverse",params:["key"]},
        "t":{action:["moveFind","moveLeft"],params:["key"]},
        "T":{action:["moveFindReverse","moveRight"],params:["key"]},
        
        "i":{action:"moveInside",params:["key"]},
        "a":{action:"moveAround",params:["key"]},
        
        "<Escape>":{action:["exitModeSelection"]},
        "v":{action:["exitModeSelection"]},
        
        "<C-u>":{action:"viewportUp"},
        "<C-d>":{action:"viewportDown"},
        
        "<A-F4>":"<Escape>:q<Enter>",
    },
    "motion": {
        "h":{action:"moveLeft"},
        "j":{action:"moveDown"},
        "k":{action:"moveUp"},
        "l":{action:"moveRight"},
        
        "w":{action:"moveWord"},
        "f":{action:"moveFind",params:["key"]},
        "i":{action:"moveInside",params:["key"]},
        "a":{action:"moveAround",params:["key"]},
        
        "gg":{action:"moveBeginning"},
        "G":{action:"moveEnd"},
        
        "w":{action:"moveWord"},
        "f":{action:"moveFind",params:["key"]},
        "F":{action:"moveFindReverse",params:["key"]},
        "t":{action:"moveTill",params:["key"]},
        "T":{action:"moveTillReverse",params:["key"]},
        
        "$":{action:"moveBack"},
        "0":{action:"moveFront"},
        "<Escape>":{action:"clearActions"},
    },
    "key": {
        "<*>":{action:"anyChar",params:["key"]},
        "<Escape>":{action:"modeNormal"},
    },
    "insert": {
        "<*>":{action:["insertChar","moveRight"],params:["key"]},
        "<Backspace>":{action:"deleteBack"},
        "<ArrowLeft>":{action:"moveLeft"},
        "<ArrowDown>":{action:"moveDown"},
        "<ArrowUp>":{action:"moveUp"},
        "<ArrowRight>":{action:"moveRight"},
        "<Escape>":{action:["modeNormal","moveLeft"]},
        "<Enter>":{action:["splitLine","moveDown","moveFront"]},
    },
    "console": {
        "<*>":{action:"insertConsoleChar",params:["key"]},
        "<A-F4>":"<Escape>:q<Enter>",
        "<Enter>":{action:"runConsole"},
        "<Backspace>":{action:"deleteBackConsole"},
        "<Escape>":{action:"modeNormal"},
    }
};

var ignoredKeys = [
   "Shift", "Alt", "Control", "<A-Alt>", "<C-Control>"
];

var replacedKeys = {
    "Spacebar": " ",
    "Backspace": "<Backspace>",
    "ArrowLeft":"<ArrowLeft>",
    "ArrowRight":"<ArrowRight>",
    "ArrowUp":"<ArrowUp>",
    "ArrowDown":"<ArrowDown>",
    "Enter":"<Enter>",
    "Escape":"<Escape>",
};

function modeNormal(data) {
    changeMode("normal");
    document.getElementById("console-text").innerText = '';
}
function modeInsert(data) {
    changeMode("insert");
}
function modeSelection(data) {
    changeMode("selection");
}
function exitModeSelection(data) {
    executeAction("toggleSelection");
    modeNormal(data);
}
function modeConsole(data) {
    document.getElementById("console-text").innerText = ':';
    changeMode("console");
}
function insertConsoleChar(data) {
    document.getElementById("console-text").innerText +=
        data.key;
    
    var consoleJson = {
        "type":"console-preview",
        "command":document.getElementById("console-text").innerText,
    };
    external.invoke(JSON.stringify(consoleJson));
}
function deleteBackConsole(data) {
    if (document.getElementById("console-text").innerText.length <= 1)
        return;
    
    document.getElementById("console-text").innerText =
        document.getElementById("console-text").innerText.slice(0, -1);
    
    var consoleJson = {
        "type":"console-preview",
        "command":document.getElementById("console-text").innerText,
    };
    external.invoke(JSON.stringify(consoleJson));
}
function runConsole(data) {
    var consoleJson = {
        "type":"console",
        "command":document.getElementById("console-text").innerText,
    };
    
    external.invoke(JSON.stringify(consoleJson));
    
    document.getElementById("console-text").innerText = '';

    changeMode("normal");
}

function numStrOverlaps(strArr, testStr) {
    var out = 0;
    for (var i = 0; i < strArr.length; i++) {
        if (strArr[i].length >= testStr.length) {
            if (strArr[i].substring(0, testStr.length) == testStr) {
                out++;
            }
        }
    }
    return out;
}

function clearActions() {
    if (actionString[0] == "" && actionString.length == 1) {
        var consoleJson = {
            "type":"console",
            "command":":cc",
        };
    
        external.invoke(JSON.stringify(consoleJson));
    }
    modeArr = [modeArr[0]];
    actionString = [""];
}

function externalDebug(msg) {
    external.invoke(JSON.stringify(
        {
            "type":"debug",
            "message":msg,
        }
    ));
}

function executeAction(action, key, motion) {
    if (window[action] != undefined) {
        window[action](executeActionJSON(action, key, motion));
    } else {
        if (typeof action == "string") {
            external.invoke(JSON.stringify(executeActionJSON(action, key, motion)));
        } else {
            for (var i = 0; i < action.length; i++) {
                executeAction(action[i], key, motion);
            }
        }
    }
}

function executeActionJSON(action, key, motion) {
    return {
        "type":"edit",
        "action":action,
        "key":key,
        "motion":motion
    };
}

function changeMode(newMode) {
    modeArr = [newMode];
    populateBuffer(currentBuffer);
    document.getElementById("mode-text").innerText =
        "-- " + newMode.toUpperCase() + " --";
}

function highlightRange(str, highlightClass, start, end, startClass, endClass) {
    if (str == "&nbsp;")
        str = "";
    if (start >= str.length || str.length == 0) {
        str = str + "<span class='" + highlightClass + " " + startClass + " " + endClass + "'> </span>";
    } else if (start == end) {
        str = str.substring(0, start) +
            "<span class='" + endClass + "'>" +
            str.substring(start, end + 1) +
            "</span>" + str.substring(end + 1);
    } else {
        if (end >= str.length) {
            str = str.substring(0, start) +
                "<span class='" + highlightClass + "'>" +
                "<span class='" + startClass + "'>" + str.substring(start, start + 1) + "</span>" +
                str.substring(start + 1, end + 1) +
                "<span class='" + endClass + "'> </span></span>";
        } else {
            str = str.substring(0, start) +
                "<span class='" + highlightClass + "'>" +
                "<span class='" + startClass + "'>" + str.substring(start, start + 1) + "</span>" + 
                str.substring(start + 1, end) +
                "<span class='" + endClass + "'>" + str.substring(end, end + 1) + "</span>" +
                "</span>" + str.substring(end + 1);
        }
    }
    return str;
}

var currentBuffer;
function populateBuffer(buffer) {
    currentBuffer = JSON.parse(JSON.stringify(buffer));
    
    //Update file name
    externalDebug(buffer.path);
    document.getElementsByClassName("tab")[0].innerText =
        (buffer.path == "" ? "<Untitled>" : buffer.path);
    
    var lines = document.querySelector("#buffer .lines");
    lines.innerHTML = "";
    
    var lineNodes = [];
    
    //Draw lines
    for (var i = 0; i < buffer.lines.length; i++) {
        var lineNode = document.createElement("p");
        lineNode.innerHTML = (buffer.lines[i] == "" ? "&nbsp;" : buffer.lines[i]);
        lineNode.classList.add("line");
        lines.appendChild(lineNode);
        lineNodes.push(lineNode);
        var lineBreakNode = document.createElement("br");
        lines.appendChild(lineBreakNode);
    }
    
    //Add cursor html
    buffer.cursors.forEach(cursor => {
        //Shift cursors to align with viewport
        cursor.line = cursor.line - buffer.viewport
        cursor.line_range = cursor.line_range - buffer.viewport
        //If not in selection mode, simply add the cursors
        if (!cursor.range) {
            if (cursor.line < lineNodes.length && cursor.line >= 0) {
                lineNodes[cursor.line].innerHTML =
                    highlightRange(lineNodes[cursor.line].innerHTML,
                        "selected-line", cursor.index, cursor.index, "cursor",
                        (modeArr[0] == "insert" ? "cursor-vbar" : "cursor"));
            }
            return;
        }
        
        var cursorPointers = [{line:cursor.line,index:cursor.index},
            {line:cursor.line_range,index:cursor.index_range}];
        cursorPointers.sort((a, b) => {
            if (a.line > b.line)
                return -1;
            if (a.line < b.line)
                return 1;
            if (a.line == b.line) {
                if (a.index > b.index)
                    return -1;
                if (a.index < b.index)
                    return 1;
            }
        });
        
        
        if (cursorPointers[0].line == cursorPointers[1].line) {
            var startClass = cursor.index > cursor.index_range ? "selected-line" : "cursor";
            var endClass = cursor.index < cursor.index_range ? "selected-line" : "cursor";
            
            lineNodes[cursorPointers[0].line].innerHTML =
                highlightRange(lineNodes[cursorPointers[0].line].innerHTML,
                    "selected-line", cursorPointers[1].index, cursorPointers[0].index, startClass, endClass);
        } else {
            if (cursorPointers[0].line < buffer.lines.length) {
                var startClass = "selected-line";
                var endClass = cursorPointers[0].line == cursor.line ? "cursor" : "selected-line";
                
                lineNodes[cursorPointers[0].line].innerHTML =
                    highlightRange(lineNodes[cursorPointers[0].line].innerHTML,
                        "selected-line", 0, cursorPointers[0].index, startClass, endClass);
            }
            if (cursorPointers[1].line < buffer.lines.length) {
                var startClass = cursorPointers[1].line == cursor.line ? "cursor" : "selected-line";
                var endClass = "selected-line";
                
                lineNodes[cursorPointers[1].line].innerHTML =
                    highlightRange(lineNodes[cursorPointers[1].line].innerHTML,
                        "selected-line", cursorPointers[1].index,
                        lineNodes[cursorPointers[1].line].innerHTML.length - 1,
                    startClass, endClass);
            }
        }
        
        for (var i = cursorPointers[1].line + 1; i <= cursorPointers[0].line - 1; i++) {
            lineNodes[i].classList.add("selected-line");
        }
    });
    
    //Do WYSIWYG text transformations
    lineNodes.forEach((lineNode, i) => {
        //MD
        if (lineNode.textContent.match(/^\# /g)) {
            lineNode.classList.add("md-h1");
        } else if (lineNode.textContent.match(/^\#\# /g)) {
            lineNode.classList.add("md-h2");
        }
        var cursorOnLine = false;
        for (var k = 0; k < buffer.cursors.length; k++) {
            if (buffer.cursors[k].line == i) {
                cursorOnLine = true;
            }
        }
        if (!cursorOnLine || true) {
            //Color
            var colorRegex = /(#([a-f]|[A-F]|[0-9]){6}|#([a-f]|[A-F]|[0-9]){3})/g;
            lineNode.innerHTML = lineNode.innerHTML.replace(colorRegex, function(match) {
                if (match.includes("span"))
                    return match;
                return "<span style='background-color:" + match + ";border-radius:4px;'>" + match + "</span>";
            });
            
            //Italics
            var italicRegex = /\*.+?\*/g;
            lineNode.innerHTML = lineNode.innerHTML.replace(italicRegex, function(match) {
                if (match.includes("span"))
                    return match;
                return "<i>" + match + "</i>";
            });
            
            //Links
            var italicRegex = /\[\[(.+?)\]\]/g;
            lineNode.innerHTML = lineNode.innerHTML.replace(italicRegex, function(match, capture, offset) {
                if (match.includes("span"))
                    return match;
                return "<span class='link'>" + match + "</span>";
            });
            
            //Latex
            var latexRegex = /\$(.+?)\$/g;
            lineNode.innerHTML = lineNode.innerHTML.replace(latexRegex, function(match, capture, offset) {
                if (match.includes("span"))
                    return match;
                //FIXME: Different output if only has spaces or error
                return katex.renderToString(capture, {
                    throwOnError: false
                });
            });
        }
    });
}

function anyChar(e) {
    if (actionString.length == 1) {
        executeAction(
            actions[modeArr[modeArr.length - 2]][actionString[actionString.length - 1]].action,
            keyString);
    } else {
        executeAction(actions[modeArr[0]][actionString[0]].action, undefined,
            executeActionJSON(
                actions[modeArr[modeArr.length - 2]][actionString[actionString.length - 1]].action,
                keyString));
    }
    clearActions();
}

function keyDown(e) {
    keyString = e.key;
    if (e.altKey) {
        keyString = "<A-" + keyString + ">";
    } else if (e.ctrlKey) {
        keyString = "<C-" + keyString + ">";
    }
    var toPress = null;
    
    if (ignoredKeys.indexOf(keyString) != -1)
        return;
    if (replacedKeys[keyString] != undefined) {
        keyString = replacedKeys[keyString];
    }
    
    mode = modeArr[modeArr.length - 1];
    modeActions = Object.keys(actions[mode]);
    
    //Wildcard mode
    if (modeActions[0] == "<*>") {
        if (modeActions.indexOf(keyString) != -1) {
            //TODO: Move this to function
            if (typeof actions[mode][keyString] == "string") {
                clearActions();
                actions[mode][keyString].match(/\<.+?\>|./g).forEach((e) => {
                    keyDown({key:e});
                });
            } else {
                executeAction(
                    actions[mode][keyString].action,
                    keyString);
            }
        } else {
            executeAction(
                actions[mode][modeActions[0]].action,
                keyString);
        }
        return;
    }
    
    //No possible actions with this key sequence
    if (numStrOverlaps(modeActions,
            actionString[actionString.length - 1] + keyString) == 0) {
        //Check and see if actionString - 1 char matches 1 modeActions.filter by len.
        var modeActionsFiltered = modeActions.filter(function (x) {
            return x.length <= actionString[actionString.length - 1].length;
        });
        
        if (numStrOverlaps(modeActionsFiltered,
            actionString[actionString.length - 1]) != 1) {
            //Still no possible action
            clearActions();
            return;
        } else {
            modeActions = modeActionsFiltered;
            toPress = keyString;
        }
    } else {
        //Append to the action string
        actionString[actionString.length - 1] += keyString;
    }
    
    //We've reached the action
    if (numStrOverlaps(modeActions, actionString[actionString.length - 1]) == 1) {
        var actionObj = actions[mode][actionString[actionString.length - 1]];
        
        if (actionObj.params == undefined)
            actionObj.params = [];
        
        if (typeof actionObj == "string") {
            clearActions();
            actionObj.match(/\<.+?\>|./g).forEach((e) => {
                keyDown({key:e});
            });
        } else if (actionObj.params.indexOf("key") != -1) {
            modeArr.push("key");
        } else if (actionObj.params.indexOf("motion") != -1) {
            modeArr.push("motion");
            actionString.push("");
        } else {
            //No pending mode, action sequence over
            if (actionString.length == 1) {
                executeAction(actions[modeArr[0]][actionString[0]].action);
            } else {
                executeAction(actions[modeArr[0]][actionString[0]].action, undefined,
                    executeActionJSON(actions[mode][actionString[actionString.length - 1]].action));
            }
            clearActions();
        }
    }
    
    if (toPress != null) {
        keyDown({key:toPress});
    }
}
