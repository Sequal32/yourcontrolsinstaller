import './stylesheet.css';
import React from 'react';
import { promisified, invoke } from 'tauri/api/tauri'

import Dialog from './components/Dialog'
import DirectoryEntry from './components/DirectoryEntry'
import OptionalFeatures from './components/FeatureList'
import Overlay from './components/Overlay'

class App extends React.Component {
    constructor(props) {
        super(props)

        this.state = {
            programDirectory: "Unknown",
            packageDirectory: "Unknown",
            browsing: false,
            installing: false,
            
            currentDialog: {
                title: "",
                description: "",
                buttonText: "",
            },
            dialogActive: false
        }
        this.selectedFeatures = []
    }

    componentDidMount() {
        promisified({
            cmd: "startup"
        }).then((args) => {

            if (args.featureList !== null) {
                args.featureList = args.featureList.map((feature) => feature["name"])
            }
            
            if (args.releaseData !== null) {
                const date = new Date(args.releaseData.date * 1000)

                this.setState({
                    currentDialog: {
                        title: args.releaseData.name,
                        description: `Released on ${date.toDateString()}`,
                        buttonText: "OK",
                    },
                    dialogActive: true
                })
            }
            

            this.setState(args)
        })
    }

    featuresCallback(selectedFeatures) {
        this.selectedFeatures = Array.from(selectedFeatures)
    }

    promptInstall() {
        if (this.state.installing) {return}

        this.setState({"installing": true})

        promisified({
            cmd: "install",
            features: this.selectedFeatures
        }).then(() => {

            this.setState({
                currentDialog: {
                    title: "Installation Successful",
                    description: "The program has been successfully installed. Get flying!",
                    buttonText: "OK"
                },
                dialogActive: true
            })

        }).catch((errorMessage) => {

            this.setState({
                currentDialog: {
                    title: "Installation Failed",
                    description: errorMessage + "\n\nMore info available in Log.txt.",
                    buttonText: "OK",
                },
                dialogActive: true
            })

        }).finally(() => {
            this.setState({installing: false})
        })
    }

    onDirectoryBrowse(type) {
        if (this.state.browsing) {return}

        this.setState({browsing: true})

        promisified({
            cmd: "browse",
            browse_for: type
        }).then((location) => {
            this.setState({[type + "Directory"]: location})
        }).finally(() => {
            this.setState({browsing: false})
        })
    }

    dialogButtonClicked() {
        this.setState({dialogActive: false})
    }

    render() {
        return (
            <div>
                  <img class="logo-image" src="logo.png"/>
                  <DirectoryEntry title="Installation Directory" location={this.state.programDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "program")}/>
                  <DirectoryEntry title="Community Packages Directory" location={this.state.packageDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "package")}/>
                  <OptionalFeatures featureList={this.state.featureList} callback={this.featuresCallback.bind(this)}/>
                  <button class="install-button" onClick={this.promptInstall.bind(this)}>{this.state.installing ? "Installing" : "Install"}</button>
                  <Overlay hidden={this.state.dialogActive}/>
                  <Dialog hidden={this.state.dialogActive} title={this.state.currentDialog.title} description={this.state.currentDialog.description} buttonText={this.state.currentDialog.buttonText} callback={this.dialogButtonClicked.bind(this)}/>
            </div>
        );
    }
}

export default App;