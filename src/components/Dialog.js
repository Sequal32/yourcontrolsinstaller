import '../stylesheet.css';
import React from 'react';

export default class Dialog extends React.Component {
    onAcknowledge() {
        if (this.props.onAck) {
            this.props.onAck()
        }
    }

    onClose() {
        if (this.props.onClose) {
            this.props.onClose()
        }
    }

    render() {
        return (
            <div class="dialog-div" style={this.props.hidden ? {"top": "25%"} : {"top": "-50%"}}>
                <h1>{this.props.title}</h1>
                <button class="dialog-x" onClick={this.onClose.bind(this)}>X</button>
                <p class="dialog-desc">{this.props.description}</p>
                <button class="generic-button dialog-button" onClick={this.onAcknowledge.bind(this)}>{this.props.buttonText}</button>
            </div>
        )
    }
}