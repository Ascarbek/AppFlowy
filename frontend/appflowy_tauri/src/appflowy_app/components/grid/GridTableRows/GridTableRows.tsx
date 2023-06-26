import { DatabaseController } from '@/appflowy_app/stores/effects/database/database_controller';
import { RowInfo } from '@/appflowy_app/stores/effects/database/row/row_cache';
import { GridTableRow } from './GridTableRow';
import { DragDropContext, Droppable, DroppableProvided } from 'react-beautiful-dnd';
import { useGridTableRows } from './GridTableRows.hooks';

export const GridTableRows = ({
  viewId,
  controller,
  allRows,
  onOpenRow,
}: {
  viewId: string;
  controller: DatabaseController;
  allRows: readonly RowInfo[];
  onOpenRow: (rowId: RowInfo) => void;
}) => {
  const { onRowsDragEnd } = useGridTableRows(controller, allRows);

  return (
    <DragDropContext onDragEnd={onRowsDragEnd}>
      <Droppable droppableId='table'>
        {(droppableProvided: DroppableProvided) => (
          <tbody ref={droppableProvided.innerRef} {...droppableProvided.droppableProps}>
            {[...allRows].reverse().map((row, i) => {
              return (
                <GridTableRow
                  onOpenRow={onOpenRow}
                  row={row}
                  key={i}
                  index={i}
                  viewId={viewId}
                  controller={controller}
                />
              );
            })}
            {droppableProvided.placeholder}
          </tbody>
        )}
      </Droppable>
    </DragDropContext>
  );
};
