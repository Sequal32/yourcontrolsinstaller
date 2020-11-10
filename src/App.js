import './stylesheet.css';
import React from 'react';
import { promisified, invoke } from 'tauri/api/tauri'

import DirectoryEntry from './components/DirectoryEntry'
import OptionalFeatures from './components/FeatureList'

class App extends React.Component {
    constructor(props) {
        super(props)

        this.state = {
            "programDirectory": "Unknown",
            "packageDirectory": "Unknown",
            "browsing": false,
            "installState": "wait"
        }
        this.selectedFeatures = null
    }

    componentDidMount() {
        promisified({
            "cmd": "startup"
        }).then((args) => {
            console.log(args)

            if (args.featureList !== null) {
                args.featureList = args.featureList.map((feature) => feature["name"])
            }

            this.setState(args)
        })
    }

    featuresCallback(selectedFeatures) {
        this.selectedFeatures = Array.from(selectedFeatures)
        console.log(this.selectedFeatures)
    }

    promptInstall() {
        if (this.state.installState != "wait" && this.state.installState != "done") {return}

        this.setState({"installState": "install"})

        promisified({
            "cmd": "install",
            "features": this.selectedFeatures
        }).then((args) => {

            this.setState({"installState": "done"})

        }).catch(() => {

            this.setState({"installState": "fail"})

        })
    }

    onDirectoryBrowse(type) {
        if (this.state.browsing) {return}

        this.setState({"browsing": true})

        promisified({
            "cmd": "browse",
            "browse_for": type
        }).then((location) => {
            this.setState({[type + "Directory"]: location})
        }).finally(() => {
            this.setState({"browsing": false})
        })
    }

    getInstallButtonText() {
        switch (this.state.installState) {
            case "wait":
                return "Install"
            case "install":
                return "Installing..."
            case "done":
                return "Done!"
            case "fail":
                return "Failed!"
        }
    }

    render() {
        return (
            <div>
                  <img class="logo-image" src="logo.png"/>
                  <DirectoryEntry title="Installation Directory" location={this.state.programDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "program")}/>
                  <DirectoryEntry title="Community Packages Directory" location={this.state.packageDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "package")}/>
                  <OptionalFeatures featureList={this.state.featureList} callback={this.featuresCallback.bind(this)}/>
                  <button class="install-button" onClick={this.promptInstall.bind(this)}>{this.getInstallButtonText()}</button>
            </div>
        );
    }
}

export default App;