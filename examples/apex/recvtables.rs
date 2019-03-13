struct RecvTable
{
	void			*inst;					//0x0000	
	RecvProp		**m_pProps;				//0x0008
	int				m_nProps;				//0x0010
	char			_unk0x0014[0x4AC];		//0x0014
	void			*m_pDecoder;			//0x04C0
	char			*m_pNetTableName;		//0x04C8
	bool			m_bInitialized;			//0x04D0
	bool			m_bInMainList;			//0x04D1
}; //Size = 0x4D2
