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
        "b":{action:"moveWordReverse"},
        
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

function encodeEntities(str) {
    return str.replace(/[\u00A0-\u9999<>\&]/g, function(i) {
        return '&#'+i.charCodeAt(0)+';';
    });
}
function decodeEntities(str) {
    return str.replace(/&#\d+?;/g, function(i) {
        return String.fromCharCode(i.match(/\d+/g)[0]);
    });
}

function applyHighlights(line, highlights) {
    if (highlights.length == 0)
        return line;
    if (line == "&nbsp;")
        line = "";
    
    var tags = [];
    var outOfBounds = false;
    //Populate tags
    for (var i = 0; i < highlights.length; i++) {
        if (highlights[i].start < line.length) {
            tags.push({tag:"<span class='" + highlights[i].class + "'>", index: highlights[i].start});
            tags.push({tag:"</span>", index: highlights[i].end + 1});
        } else if (line.length == 0) {
            tags.push({tag:"<span class='" + highlights[i].class + "'>", index: 0});
            tags.push({tag:"</span>", index: 1});
        } else {
            outOfBounds = true;
            tags.push({tag:"<span class='" + highlights[i].class + "'>", index: line.length});
            tags.push({tag:"</span>", index: line.length + 1});
        }
    }
    if (line == "" || outOfBounds)
        line = line + " ";
    
    //Sort highest index to lowest index
    tags.sort(function(a, b) {
        var n = b.index - a.index;
        if (n !== 0)
            return n;
        
        if (a.tag != "</span>" && b.tag == "</span>") {
            return -1;
        }
        return 1;
    });
    //Apply tags to line
    for (var i = 0; i < tags.length; i++) {
        line = line.substring(0, tags[i].index) +
            tags[i].tag +
            line.substring(tags[i].index);
    }
    return line;
}

var currentBuffer;
//FIXME: Don't repopulate entire buffer every keypress
function populateBuffer(buffer) {
    //FIXME: Find better way to clone buffer
    currentBuffer = JSON.parse(JSON.stringify(buffer));
    
    //Update file name
    externalDebug(buffer.path);
    document.getElementsByClassName("tab")[0].innerText =
        (buffer.path == "" ? "<Untitled>" : buffer.path);
    
    var lines = document.querySelector("#buffer .lines");
    lines.innerHTML = "";
    
    var lineNodes = [];
    var highlightRanges = [];
    
    //Draw lines
    for (var i = 0; i < buffer.lines.length; i++) {
        var lineNode = document.createElement("p");
        lineNode.innerHTML = (buffer.lines[i] == "" ? "&nbsp;" : buffer.lines[i]);
        lineNode.classList.add("line");
        lines.appendChild(lineNode);
        
        lineNodes.push(lineNode);
        highlightRanges.push([]);
        
        var lineBreakNode = document.createElement("br");
        lines.appendChild(lineBreakNode);
    }
    
    //Add cursor html
    buffer.cursors.forEach(cursor => {
        //Shift cursors to align with viewport
        cursor.line = cursor.line - buffer.viewport
        cursor.line_range = cursor.line_range - buffer.viewport
        
        if (cursor.line < lineNodes.length && cursor.line >= 0) {
            highlightRanges[cursor.line].push(
                {class: (modeArr[0] == "insert" ? "cursor-vbar" : "cursor"), start: cursor.index, end: cursor.index});
        }
        if (!cursor.range)
            return;
        
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
            highlightRanges[cursor.line].push(
                {class:"selected-line", start: cursorPointers[1].index, end: cursorPointers[0].index});
        } else {
            if (cursorPointers[0].line < buffer.lines.length) {
                highlightRanges[cursorPointers[0].line].push(
                    {class:"selected-line", start: 0, end: cursorPointers[0].index});
            }
            if (cursorPointers[1].line < buffer.lines.length) {
                highlightRanges[cursorPointers[1].line].push(
                    {class:"selected-line", start: cursorPointers[1].index, end: lineNodes[cursorPointers[1].line].innerHTML.length - 1});
            }
        }
        
        for (var i = cursorPointers[1].line + 1; i <= cursorPointers[0].line - 1; i++) {
            lineNodes[i].classList.add("selected-line");
        }
    });
    
    //Do WYSIWYG text transformations
    lineNodes.forEach((lineNode, i) => {
        //Apply highlights
        lineNode.innerHTML = applyHighlights(lineNode.innerHTML, highlightRanges[i]);
        
        //MD
        if (lineNode.textContent.match(/^\# /g)) {
            lineNode.classList.add("md-h1");
        } else if (lineNode.textContent.match(/^\#\# /g)) {
            lineNode.classList.add("md-h2");
        }

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
