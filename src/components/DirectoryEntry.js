import React from 'react';
import '../stylesheet.css';

class DirectoryEntry extends React.Component {
    constructor(props) {
        super(props)

        this.state = {}
        this.browsing = false
    }

    browseHover() {
        document.body.style.cursor = "pointer"
        this.setState({"hover": true})
    }

    browseLeave() {
        document.body.style.cursor = "default"
        this.setState({"hover": false})
    }

    browseClick() {
        this.props.onBrowse()
    }

    render() {
        return (
            <div class="directory-div">
                <p class="overtop-label">{this.props.title}</p>
                <div class="directory-inner-div">
                    <p class="directory-text">{this.props.location}</p>
                    <img class="browse-button" src={this.state.hover ? "folder-open.svg" : "folder.svg"} onMouseEnter={this.browseHover.bind(this)} onMouseLeave={this.browseLeave.bind(this)} onMouseDown={this.browseClick.bind(this)} alt="browse"></img>
                </div>
            </div>
        );
    }
}

export default DirectoryEntry;