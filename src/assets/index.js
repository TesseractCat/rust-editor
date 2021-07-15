//TODO: Split this into multiple file using "browserify" or "webpack"
window.addEventListener('load', function() {
    external.invoke(JSON.stringify({"type":"init"}));
    document.focus();
});
window.addEventListener('focus', function () {
    document.focus();
});
document.addEventListener('keydown', keyDown);

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
    //togglePalette(false);
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
    //togglePalette(true);
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

function externalDebug(msg) {
    external.invoke(JSON.stringify(
        {
            "type":"debug",
            "message":msg,
        }
    ));
}

function executeKeyEvent(event, key) {
    external.invoke(JSON.stringify({
        "type":"keyevent",
        "event":event,
        "key":key,
    }));
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

function entitySubstring(str, start, end) {
    var htmlEntity = /&#\d+;/g;
    
    var match;
    while ((match = htmlEntity.exec(str)) !== null) {
        if (match.index < start)
            start -= match[0].length - 1;
        if (match.index < end)
            end -= match[0].length - 1;
    }
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
    document.getElementsByClassName("tab")[0].innerText =
        (buffer.path == "" ? "<Untitled>" : buffer.path);
    
    var lines = document.querySelector("#buffer .lines");
    while (lines.firstChild)
        lines.removeChild(lines.firstChild);
    
    var lineNodes = [];
    var highlightRanges = [];
    
    //Draw lines
    for (var i = 0; i < buffer.lines.length; i++) {
        var lineNode = document.createElement("p");
        lineNode.innerHTML = (buffer.lines[i] == "" ? "&nbsp;" : buffer.lines[i]);
        lineNode.classList.add("line");
        
        lineNodes.push(lineNode);
        highlightRanges.push([]);
    }
    
    var linesWithCursor = [];
    
    //Add cursor html
    buffer.cursors.forEach(cursor => {
        linesWithCursor.push(cursor.line);
        linesWithCursor.push(cursor.line_range);
        
        if (cursor.line < lineNodes.length && cursor.line >= 0) {
            highlightRanges[cursor.line].push(
                {class: (false ? "cursor-vbar" : "cursor"), start: cursor.index, end: cursor.index});
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
        var appendLineNode = true;
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
            if (match.includes("<span") || match.includes("span>"))
                return match;
            return "<span style='background-color:" + match + ";border-radius:4px;'>" + match + "</span>";
        });
        
        //Emphasis
        var emphasisRegex = /(\*+)(.*?)\1/g;
        lineNode.innerHTML = lineNode.innerHTML.replace(emphasisRegex, function(match) {
            if (match.includes("<span") || match.includes("span>"))
                return match;
            if (match.startsWith("**")) {
                return "<b>" + match + "</b>";
            }
            return "<i>" + match + "</i>";
        });
        
        //Links
        var linkRegex = /\[\[(.+?)\]\]/g;
        lineNode.innerHTML = lineNode.innerHTML.replace(linkRegex, function(match, capture, offset) {
            if (match.includes("<span") || match.includes("span>"))
                return match;
            return "<span class='link'>" + match + "</span>";
        });
        
        //Latex
        var latexRegex = /\$(.+?)\$/g;
        lineNode.innerHTML = lineNode.innerHTML.replace(latexRegex, function(match, capture, offset) {
            if (match.includes("<span") || match.includes("span>"))
                return match;
            //FIXME: Different output if only has spaces or error
            return katex.renderToString(capture, {
                throwOnError: false
            });
        });
        
        ////Tables
        //var tableRegex = /^(\|.+)+/g;
        //if (tableRegex.test(lineNode.innerHTML) && linesWithCursor.indexOf(i) == -1) {
        //    tableRowNode = document.createElement("tr");
        //    tableRowNode.innerHTML = lineNode.innerHTML;
        //    tableRowNode.innerHTML = tableRowNode.innerHTML.replace(tableRegex, function(match, capture, offset) {
        //        var items = match.split("|").filter(x => x != "");
        //        return "<td>" + items.join("</td><td>") + "</td>";
        //    });
        //    
        //    if (lineNodes[i - 1] != null && lineNodes[i - 1].tagName == "TABLE") {
        //        lineNodes[i - 1].appendChild(tableRowNode);
        //        lineNodes[i] = lineNodes[i - 1];
        //        appendLineNode = false;
        //    } else {
        //        tableNode = document.createElement("TABLE");
        //        tableNode.appendChild(tableRowNode);
        //        lineNodes[i] = tableNode;
        //        lineNode = tableNode;
        //    }
        //}
        
        if (appendLineNode) {
            lines.appendChild(lineNode);
            var lineBreakNode = document.createElement("br");
            lines.appendChild(lineBreakNode);
        }
    });
}

function populateLine(idx, new_line) {
    //TODO
}

function togglePalette(visible) {
    palette.style.visibility = visible ? "visible" : "hidden";
    palette.style.opacity = visible ? 1 : 0;
    if (visible) {
        window.setTimeout(function() {
            document.getElementById("palette-input").focus();
        }, 10);
    }
}

function keyDown(e) {
    if (document.activeElement === document.getElementById("palette-input")) {
        return;
    }
    
    keyString = e.key;
    if (e.altKey) {
        keyString = "<A-" + keyString + ">";
    } else if (e.ctrlKey) {
        keyString = "<C-" + keyString + ">";
    }
    executeKeyEvent("keydown", keyString);
}
