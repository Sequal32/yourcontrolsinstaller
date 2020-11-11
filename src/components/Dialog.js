import '../stylesheet.css';
import React from 'react';

export default class Dialog extends React.Component {
    onAcknowledge() {
        if (this.props.callback) {
            this.props.callback()
        }
    }

    render() {
        return (
            <div class="dialog-div" style={this.props.hidden ? {"top": "25%"} : {"top": "-50%"}}>
                <h1>{this.props.title}</h1>
                <p>{this.props.description}</p>
                <button class="dialog-button" onClick={this.onAcknowledge.bind(this)}>{this.props.buttonText}</button>
            </div>
        )
    }
}