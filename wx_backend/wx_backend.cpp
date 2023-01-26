#include <wx/wx.h>

void (*func_pointer)();

void print_hello_world() {
    std::cout << "Hello, World! {from func_pointer 2x} xD" << std::endl;
}

extern "C" {
    void set_func_pointer(void (*func)()) {
        func_pointer = func;
    }

    void start_wx_system(int argc, char **argv) {
        //func_pointer = &print_hello_world;

        wxEntryStart(argc, argv);
        wxTheApp->CallOnInit();
        //wxTheApp->MainLoop();

        // cleaning up...
        //wxTheApp->OnExit();
        //wxEntryCleanup();
    }

    void update_events_loop() {
        wxTheApp->SafeYield(wxTheApp->GetTopWindow(), false);
    }
}

class CoreFrame : public wxFrame {
public:
    CoreFrame(const wxString& title) : wxFrame(NULL, wxID_ANY, title) {

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
    CoreFrame *coreFrame;

    virtual bool OnInit() {
        coreFrame = new CoreFrame("assur_ui App");

        func_pointer();

        coreFrame->Show();

        std::cout << "I am currently on init@|@" << std::endl;
        return true;
    }
};


wxIMPLEMENT_APP_NO_MAIN(MyApp);




/*
    Commented out code guides:

    //First:
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

    //Second:
        wxButton* button = new wxButton(this, wxID_OK, "OK");
        wxTextCtrl* textBox = new wxTextCtrl(this, wxID_ANY);

        // Set up the sizer
        wxBoxSizer* sizer = new wxBoxSizer(wxVERTICAL);
        sizer->Add(textBox, 1, wxEXPAND);
        sizer->Add(button, 0, wxALIGN_RIGHT | wxALL, 10);
        SetSizer(sizer);

        // Set up the event handlers
        Bind(wxEVT_MENU, &CoreFrame::OnExit, this, wxID_EXIT);
        Bind(wxEVT_BUTTON, &CoreFrame::OnOK, this, wxID_OK);
*/