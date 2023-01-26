#include <wx/wx.h>

class MyFrame : public wxFrame {
public:
    MyFrame(const wxString& title) : wxFrame(NULL, wxID_ANY, title) {
        // Set up the menu bar

        // Create the button and text box
        wxButton* button = new wxButton(this, wxID_OK, "OK");
        wxTextCtrl* textBox = new wxTextCtrl(this, wxID_ANY);

        // Set up the sizer
        wxBoxSizer* sizer = new wxBoxSizer(wxVERTICAL);
        sizer->Add(textBox, 1, wxEXPAND);
        sizer->Add(button, 0, wxALIGN_RIGHT | wxALL, 10);
        SetSizer(sizer);

        // Set up the event handlers
        Bind(wxEVT_MENU, &MyFrame::OnExit, this, wxID_EXIT);
        Bind(wxEVT_BUTTON, &MyFrame::OnOK, this, wxID_OK);
    }

private:
    void OnExit(wxCommandEvent& event) {
        Close();
    }

    void OnOK(wxCommandEvent& event) {
        // Do something when the button is clicked
    }
};

class MyApp : public wxApp {
public:
    virtual bool OnInit() {
        MyFrame* frame = new MyFrame("My App");
        //frame->SetMenuBar(new wxMenuBar());


        wxMenuBar* menuBar = new wxMenuBar();
        wxMenu* fileMenu = new wxMenu();
        fileMenu->Append(wxID_EXIT);
        menuBar->Append(fileMenu, "&File");
        wxMenu* editMenu = new wxMenu();
        editMenu->Append(wxID_CUT);
        editMenu->Append(wxID_COPY);
        editMenu->Append(wxID_PASTE);
        menuBar->Append(editMenu, "&Edit");
        frame->SetMenuBar(menuBar);

        frame->Show();

        std::cout << "I am currently on init@" << std::endl;
        return true;
    }
};




int main(int argc, char **argv){

    std::cout << "Hello, World! xD" << std::endl;

    wxEntryStart(argc, argv);
    wxTheApp->CallOnInit();
    wxTheApp->OnRun();



	return 0;
}

wxIMPLEMENT_APP_NO_MAIN(MyApp);
