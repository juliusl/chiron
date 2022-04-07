port module Main exposing (..)

import Browser
import Html exposing (..)
import Element exposing (..)
import Element.Input
import Markdown
import Layout
import Editor
import Instructions

main =
    Browser.document
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- MODEL

type alias Model =
  { editor: Editor
  , instructions: String
  }

type alias Editor =
    { text: String, language: String, saved: String }
    
type Msg 
    = ResetText
    | Dispatch String
    | Save String
    | Instructions String
    | Done


init : () -> ( Model, Cmd Msg )
init _  =
    ( Model {text=sampleLab, language="markdown", saved=""} sampleLab, Cmd.none )

-- UPDATE

update : Msg -> Model -> ( Model, Cmd msg)
update msg model =
    let
        editor = model.editor
    in case msg of 
        ResetText -> 
            ({model | editor = { editor | text = editor.saved}}, Cmd.none)
        Save content -> 
            ({model | editor = { editor | text = content, saved = content}, instructions = content}, Cmd.none)
        Dispatch cmd -> 
            ( model, dispatchEditorCmd cmd )
        Instructions instructions ->
            ( { model | instructions = instructions }, Cmd.none )
        Done -> 
            ( { model | instructions = model.editor.text }, Cmd.none )

-- SUBSCRIPTIONS

-- If Monaco is enabled, this will allow us to pipe commands to the editor
port dispatchEditorCmd : String -> Cmd msg
-- This is called by monaco to pass the current value of it's editor
port saveContent : (String -> msg) -> Sub msg

subscriptions : Model -> Sub Msg
subscriptions _ =
  saveContent Save

-- VIEW

view : Model -> Browser.Document Msg 
view model = 
    {
        title = "Chiron Portal",
        body = [
            Layout.view 
                { title = "Editor"
                , content = viewCodeEditor { enableMonaco = True, model = model }
                , detail = Instructions.viewInstructions onNext Done model.instructions }
        ]
    }

onNext: List String -> Maybe Msg
onNext remaining = 
    if List.isEmpty remaining then 
        Nothing
    else
        Just (Instructions (String.join "\n" remaining))

viewCodeEditor: { enableMonaco: Bool, model: Model } -> Element Msg
viewCodeEditor settings =
    let
        enableMonaco = settings.enableMonaco
        model = settings.model
        editor = 
            { language = model.editor.language
            , text = model.editor.text }
    in
        if enableMonaco then
            Element.column [width fill, height fill] 
            [ Element.Input.button [] 
                { onPress=Just (Dispatch "save")
                , label=(Element.text "Render") 
                }
            , Editor.viewMonacoEditor editor
            ]
        else 
            Editor.viewMultilineEditor Save editor

-- Test Methods 

viewSample: String -> Element.Element msg 
viewSample markdown = 
    case Markdown.viewMarkdown markdown of
        Ok rendered -> 
            Element.column [spacing 20] rendered 
        
        Err errors -> 
            Element.text errors 

sampleComponent : String 
sampleComponent = """
# Component - Kubernetes in Docker (KIND)
```yaml
id: "kind"
display_name: "Install KIND"
description: "Installing KIND"
depends_on:
- golang
tools:
  cloud-init:
  - install-kind.yml:jinja2
```
"""

sampleLab : String 
sampleLab = """
# Lab - Setup a dev-box 
```yaml
id: "devbox-setup-2"
display_name: "Devbox Setup"
description: "Learn how to setup your own devbox"
depends_on:
- kind
tools:
- az_cli:
  - az vm create
```
## Generate an ssh key
```yaml
requires:
- id_rsa_pub:
tools:
- bash:
  - ssh-keygen
```
"""
